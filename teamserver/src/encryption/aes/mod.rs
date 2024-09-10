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

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

pub struct Aes;

impl Aes {

    pub fn generate() -> [u8; 32] {
        // Generating an AES key to be used in the encryption
        let key: [u8; 32] = Aes256Gcm::generate_key(OsRng).into();
        key
    }

    pub fn encrypt(key: [u8; 32], data: Vec<u8>) -> Vec<u8> {
        // Encrypting data using Authenticated AES.
        let mut encrypted_data: Vec<u8> = Vec::new();
        let cipher = Aes256Gcm::new((&key).into());

        // When using only an AES key instead of an AES key and a nonce, the same two plaintext commands will
        // result in the same encrypted text. When using a nonce this will add randomization to the ciphertext
        // As a result two the same plaintext commands will result in two different ciphertexts.

        // A nonce is typically either concatted to the ciphertext or transmitted seperataly.
        // An idea would also be to encrypt the nonce using the RSA public key
        let nonce: [u8; 12] = Aes256Gcm::generate_nonce(&mut OsRng).into();

        if let Ok(ciphertext) = cipher.encrypt((&nonce).into(), data.as_ref()) {
            ciphertext.iter().for_each(|i| encrypted_data.push(*i));
            nonce.iter().for_each(|i| encrypted_data.push(*i));
            return encrypted_data;
        }
        encrypted_data
    }

    pub fn decrypt(key: [u8; 32], data: Vec<u8>) -> Vec<u8> {
        let cipher = Aes256Gcm::new((&key).into());
        
        // A nonce is a constant of 12 bytes, however an empty message "" will result in 16 bytes of ciphertext
        if data.len() >= 16 {
            let nonce = &data[data.len() - 12..];
            
            if let Ok(plaintext) = cipher.decrypt((nonce).into(), &data[..data.len() - 12]) {
                return plaintext;
            }

        }
        vec![]
    }
}