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

use crate::encoding::base64::Base64;
use crate::keys::Keys;

use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rsa::pkcs8::{EncodePublicKey, EncodePrivateKey, DecodePublicKey};
use rsa::pkcs1::{
    EncodeRsaPublicKey, EncodeRsaPrivateKey, LineEnding,
    DecodeRsaPublicKey, DecodeRsaPrivateKey
};
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::sha2::{Digest, Sha256};

pub struct Rsa;

impl Rsa {
    pub fn generate(bits: usize) -> Result<RsaPrivateKey> {
        // Generation of RSA Private key

        let mut rng = rand::thread_rng();
        Ok(RsaPrivateKey::new(&mut rng, bits)?)
    }

    pub fn public_key_from_private_key(private_key: RsaPrivateKey) -> RsaPublicKey {
        // Retrieving a public key belonging to a given private key

        RsaPublicKey::from(&private_key)
    }

    pub fn export_private_to_pem(private_key: RsaPrivateKey, name: &str) -> Result<()> {
        // Saving the Private key to disk to be saved in case of an application interruption.

        Ok(private_key.write_pkcs1_pem_file(name, LineEnding::default())?)
    }

    pub fn export_public_to_pem(public_key: RsaPublicKey, name: &str) -> Result<()> {
        // Saving the Public key to disk to be saved in case of an application interruption.

        Ok(public_key.write_pkcs1_pem_file(name, LineEnding::default())?)
    }

    pub fn load_private_key(name: &str) -> Result<RsaPrivateKey> {
        // Loading the private key from a PEM file

        Ok(RsaPrivateKey::read_pkcs1_pem_file(Keys::get_key_location(name)?)?)
    }

    pub fn load_public_key(name: &str) -> Result<RsaPublicKey> {
        // Loading a public key from a PEM file

        Ok(RsaPublicKey::read_pkcs1_pem_file(Keys::get_key_location(name)?)?)
    }

    pub fn public_key_to_string(public_key: RsaPublicKey) -> Result<String> {
        // Transforming the public key to sendable data using URL safe base64 encoding

        let binding = public_key.to_public_key_der()?;
        let public_key_der = binding.as_bytes();
        Ok(Base64::encode(public_key_der))
    }

    pub fn private_key_to_string(private_key: RsaPrivateKey) -> Result<String> {
        let binding = private_key.to_pkcs8_der()?;
        let private_key_der = binding.as_bytes();
        let private_key_der_base64 = Base64::encode(private_key_der);
        Ok(private_key_der_base64)
    }

    pub fn public_key_string_to_struct(public_key: String) -> Result<RsaPublicKey> {
        let public_der = Base64::decode(&public_key);
        let public_key = RsaPublicKey::from_public_key_der(&public_der)?;
        Ok(public_key)
    }

    pub fn encrypt(public_key: RsaPublicKey, data: Vec<u8>) -> Result<Vec<u8>> {
        // Encrypting data using RSA with a RSA public key

        let mut rng = rand::thread_rng();
        let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])?;
        Ok(encrypted_data)
    }

    pub fn decrypt(private_key: RsaPrivateKey, enc_data: Vec<u8>) -> Result<Vec<u8>> {
        // Decrypting data that was encrypted using the teamservers public RSA key

        let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, &enc_data)?;
        Ok(decrypted_data)
    }

    pub fn verify(public_key: RsaPublicKey, data: Vec<u8>, signature: &[u8]) -> bool {
        // Verifying a signature by it's public key

        let padding = Pkcs1v15Sign::new::<Sha256>();
        let digest = sha256_hash(data);

        if let Ok(_) = public_key.verify(padding, &digest, signature) {
            return true;
        }
        false
    }

    pub fn sign(private_key: RsaPrivateKey, data: Vec<u8>) -> Result<Vec<u8>> {
        // Signing a message for integrity and authenticity using a private key

        let padding = Pkcs1v15Sign::new::<Sha256>();
        let digest = sha256_hash(data);
        let signature = private_key.sign(padding, &digest)?;
        Ok(signature)
    }
}

fn sha256_hash(input: Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let digest = hasher.finalize();
    digest.to_vec()
}