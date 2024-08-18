
use crate::communication::{HttpConfiguration, PipeConfiguration, TcpConfiguration};

// Enum with all the communication methods from /src/communication/mod.rs
pub enum CommunicationMethod {
    Http(HttpConfiguration),
    Pipe(PipeConfiguration),
    Tcp(TcpConfiguration),
}

// Structure for the Beacon communication
pub struct BeaconCom {
    pub host: String,
    pub port: u64,
    pub method: String,
    pub methodinfo: CommunicationMethod,
}

// Structure for the Beacon configuration
pub struct Configuration {
    pub beacon: BeaconCom,
    pub sleeptime: u64,
    pub jitter: u64,
    pub pubkey: String,
    pub aes_key: [u8; 32],
    pub pid: u64,
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
    pub fn new() {
        todo!();
    }
}