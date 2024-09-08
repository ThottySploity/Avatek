use crate::encryption::rsa::Rsa;
use crate::utilities::Utils;

use log::{error, info};

pub struct Keys;

impl Keys {
    pub fn exists() -> bool {
        // Checks the existance of all paths
        // If one does not exist it returns false

        key_locations().into_iter().all(|path| Utils::is_path_valid(&path))
    }

    pub fn generate(force: bool) {
        // Generates all the keys for Avatek to function

        for kloc in key_locations() {
            // If a key location does not exist, it will create it.
            if !Utils::is_path_valid(&kloc) {
                if Utils::create_path(&kloc) {
                    info!("Created new folder: {}", kloc)
                }
            }

            for kname in key_names() {
                let key = format!("{}{}", kloc, kname);
                if Utils::is_path_valid(&key) && !force {
                    error!("{} already exists, overwriting can mean losing access to beacons. Run the program with -f flag to overwrite", key);
                    return;
                }
            }

            info!("Generating private and public keypair (2048 bits) for: {}", kloc);
            if let Ok(private_key) = Rsa::generate(2048) {
                let public_key = Rsa::public_key_from_private_key(private_key.clone());
                
                for kname in key_names() {
                    let key = format!("{}{}", kloc, kname);
                    if kloc.contains(&remove_prefix_and_suffix(kname.clone())) {

                        if key.contains("private") {
                            if let Ok(_) = Rsa::export_private_to_pem(private_key.clone(), &key) {
                                info!("Exported: {}", key);
                            }
                        } else {
                            if let Ok(_) = Rsa::export_public_to_pem(public_key.clone(), &key) {
                                info!("Exported: {}", key);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn key_locations() -> Vec<String> {
    // This function holds all the windows specific folders

    vec![
        "keys\\teamclient\\".to_string(),
        "keys\\teamserver\\".to_string(),
        "keys\\beacons\\".to_string(),
    ]
}

#[cfg(target_os = "linux")]
fn key_locations() -> Vec<String> {
    // This function holds all the Linux specific folders

    vec![
        "keys/teamclient/".to_string(),
        "keys/teamserver/".to_string(),
        "keys/beacons/".to_string(),
    ]
}

fn key_names() -> Vec<String> {
    // These are the same cross platform, that's why it does not require a 
    // target_os cfg 

    vec![
        // Keypair for the teamclient
        "private_key_teamclient.pem".to_string(),
        "public_key_teamclient.pem".to_string(),

        // Keypair for the teamserver
        "private_key_teamserver.pem".to_string(),
        "public_key_teamserver.pem".to_string(),

        // Keypair for the beacons
        "private_key_beacon.pem".to_string(),
        "public_key_beacon.pem".to_string(),
    ]
}

fn remove_prefix_and_suffix(input: String) -> String {
    input.replace("private_key_", "").replace("public_key_", "").replace(".pem", "").to_string()
}