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
use crate::utilities::encoding::base64::Base64;
use anyhow::{anyhow, Result};

use rsa::{RsaPublicKey, Pkcs1v15Encrypt, pkcs8::DecodePublicKey};

pub struct Rsa {
    public_key: String,
    data: Vec<u8>,
}

impl Rsa {
    pub fn new(public_key: String, data: Vec<u8>) -> Self {
        Self {
            public_key: public_key,
            data: data,
        }
    }

    pub fn encrypt(&self) -> Result<Vec<u8>> {
        // This function is responsible for encrypting data using RSA public key

        if let Ok(public_key) = pubkey_string_to_struct(self.public_key.clone()) {
            let mut rng = rand::thread_rng();
            
            if let Ok(enc_data) = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &self.data[..]) {
                return Ok(enc_data);
            }
        }
        Err(anyhow!("Failed to RSA encrypt"))
    }
}

fn pubkey_string_to_struct(public_key: String) -> Result<RsaPublicKey> {
    // Transforming an base64 encoded public key to the RsaPublicKey struct
    let public_der = Base64::decode(&public_key);
    let public_key = RsaPublicKey::from_public_key_der(&public_der)?;
    Ok(public_key)
}