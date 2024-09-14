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

// Code for retrieving command from body
pub fn retrieve_command(body: String, private_key: RsaPrivateKey) -> Result<String> {
    let (encrypted_data, encrypted_key, _) = retrieve_info(body)?;
    let decrypted_key = Rsa::decrypt(private_key.clone(), encrypted_key)?;
    let decrypted_command = Aes::decrypt(Aes::transform(decrypted_key), encrypted_data);
    Ok(Utils::convert(&decrypted_command)?.to_string())
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