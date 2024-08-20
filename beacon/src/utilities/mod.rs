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

pub mod encryption;
pub mod encoding;

pub struct Utilities;


use anyhow::Result;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;

impl Utilities {
    pub fn vec_to_key_array(input: Vec<u8>) -> [u8; 32] {
        let mut output = [0u8; 32];

        if input.len() >= 32 {
            for i in 0..input.len() {
                output[i] = input[i];
            }
        }

        output
    }

    pub fn strip_escape_chars(input: String) -> String {
        input.replace("\"", "").to_string()
    }

    pub fn pointer_to_constant_wide(str: &str) -> Vec<u16> {
        OsString::from(str)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }

    pub fn get_executable_path() -> Result<String> {
        // Returns the current executables path
        Ok(std::env::current_exe()?.display().to_string())
    }
}