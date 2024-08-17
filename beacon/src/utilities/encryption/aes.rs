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

use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{typenum::U16, GenericArray},
};

use rand::rngs::OsRng;
use rand::RngCore;

pub struct Aes {
    key: [u8; 32],
    buffer: Vec<u8>,
}

impl Aes {
    pub fn new(buffer: Vec<u8>) -> Self {
        // Initialising a new AES struct for future encryption/decryption of data
        Self {
            key: generate_key(),
            buffer: buffer,
        }
    }

    pub fn buffer(&self) -> Vec<u8> {
        // Getting the buffer out of the AES struct
        self.buffer.to_owned()
    }

    pub fn key(&self) -> [u8; 32] {
        // Getting the key out of the AES struct
        self.key
    }

    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        // Manually setting the buffer to something else
        // Helpful for decrypting already encrypted buffers
        self.buffer = buffer
    }

    pub fn set_key(&mut self, key: [u8; 32]) {
        // Manually setting the key to something predefined
        // Helpful for decrypting or encrypting buffers with known keys
        self.key = key
    }

    pub fn encrypt(&self) -> Vec<u8> {
        // Encryption function used to encrypt a buffer using AES 256
        let mut output = Vec::new();

        let mut index = 0;

        for _ in 0..self.buffer.len() / 16 {
            for i in encrypt_block_aes(self.key, &self.buffer[index..index+16]) {
                output.push(i);
            }
            index = index + 16;
        }

        if self.buffer.len() % 16 > 0 {
            for i in encrypt_block_aes(self.key, &self.buffer[index..index+self.buffer.len() % 16]) {
                output.push(i);
            }
        }

        output
    }

    pub fn decrypt(&self) -> Vec<u8> {
        // Decryption function used to decrypt AES 256 encrypted buffers
        let mut output = Vec::new();

        let mut index = 0;

        for _ in 0..self.buffer.len() / 16 {
            for i in decrypt_block_aes(self.key, &self.buffer[index..index+16]) {
                output.push(i);
            }
            index = index + 16;
        }

        if self.buffer.len() % 16 > 0 {
            for i in decrypt_block_aes(self.key, &self.buffer[index..index+self.buffer.len() % 16]) {
                output.push(i);
            }
        }

        // The padding is still present in the decrypted bytes
        // To remove this padding we iter over the output and skipping all 144 bytes (which are NOP bytes (0x90))
        let clean_buffer: Vec<u8> = output.clone().into_iter().filter(|&x| x != 144).collect();
        clean_buffer
    }
}

fn generate_key() -> [u8; 32] {
    // Generation of random AES key
    let mut keyword = [0u8; 32];
    OsRng.fill_bytes(&mut keyword);
    keyword
}

fn encrypt_block_aes(key: [u8; 32], input: &[u8]) -> Vec<u8> {
    // Responsible for encrypting a block of 16 bytes (or less but this gets padded)
    let cipher = Aes256::new(&GenericArray::clone_from_slice(&key));
    let padding = add_padding(input.to_vec());
    let mut bytes: GenericArray<_, U16> = GenericArray::clone_from_slice(&padding[0..16]);
    cipher.encrypt_block(&mut bytes);
    bytes.to_vec()
}

fn decrypt_block_aes(key: [u8; 32], input: &[u8]) -> Vec<u8> {
    // Responsible for decrypting a block of 16 bytes (also gets added padding)
    let cipher = Aes256::new(&GenericArray::clone_from_slice(&key));
    let padding = add_padding(input.to_vec());
    let mut bytes: GenericArray<_, U16> = GenericArray::clone_from_slice(&padding[0..16]);
    cipher.decrypt_block(&mut bytes);
    bytes.to_vec()
}

fn add_padding(input: Vec<u8>) -> Vec<u8> {
    // Adds padding to buffer to make it encryptable
    let mut output = Vec::new();

    if input.len() < 16 {
        let padding_required = 16 - input.len();
        for _i in 0..padding_required {
            output.push(0x90);
        }
    }

    for i in input {
        output.push(i);
    }

    output
}