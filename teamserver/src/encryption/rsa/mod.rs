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

use anyhow::Result;

use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPublicKey, EncodeRsaPrivateKey, LineEnding};

pub struct Rsa;

impl Rsa {
    pub fn generate(bits: usize) -> Result<RsaPrivateKey> {
        let mut rng = rand::thread_rng();
        Ok(RsaPrivateKey::new(&mut rng, bits)?)
    }

    pub fn public_key_from_private_key(private_key: RsaPrivateKey) -> RsaPublicKey {
        RsaPublicKey::from(&private_key)
    }

    pub fn export_private_to_pem(private_key: RsaPrivateKey, name: &str) -> Result<()> {
        Ok(private_key.write_pkcs1_pem_file(name, LineEnding::default())?)
    }

    pub fn export_public_to_pem(public_key: RsaPublicKey, name: &str) -> Result<()> {
        Ok(public_key.write_pkcs1_pem_file(name, LineEnding::default())?)
    }
}