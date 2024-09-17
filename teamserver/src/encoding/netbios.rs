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

pub struct Netbios;

impl Netbios {
    pub fn encode(input: &[u8]) -> String {
        // Encoding bytes into a Netbios encoding representation

        input.into_iter().map(|c| {
            let ord_c = *c as u8;
            format!("{}{}", ((ord_c >> 4) + b'A') as char, ((ord_c & 0xF) + b'A') as char)
        }).collect()
    }

    pub fn decode(input: String) -> Vec<u8> {
        // Decoding a netbios encoding representation to it's Vec<u8> 
        
        let binding = input.to_uppercase();
        let mut chars = binding.chars().peekable();
        let mut result = Vec::new();
        
        while let Some(c) = chars.next() {
            if let Some(next_c) = chars.next() {
                // Decode the two characters back to a single byte
                let high_nibble = (c as u8 - b'A') << 4;
                let low_nibble = next_c as u8 - b'A';
                let byte = high_nibble | low_nibble;
                result.push(byte);
            } else {
                // Handle the case where there's an odd number of characters
                // This can be treated as an error or you can handle it as needed
                return Vec::new();
            }
        }
        
        result
    }
}