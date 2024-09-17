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

pub mod beacondecoder;

use crate::webserver::endpoints::listeners::metadata::MetaData;

use anyhow::Result;
use log::error;

use std::path::Path;
use std::fs;

pub struct Utils;

impl Utils {

    pub fn is_path_valid(path: &str) -> bool {
        // Validates if a given path exists on the machine
        Path::new(&path).exists()
    }

    pub fn create_path(path: &str) -> bool {
        // Creates a directory
        if let Err(e) = fs::create_dir_all(path) {
            error!("Failed creating path: {} with error: {}", path, e);
            return false;
        }
        true
    }

    pub fn convert(input: &[u8]) -> Result<String> {
        // Convert bytes to its UTF8 representation
        Ok(std::str::from_utf8(&input)?.to_string())
    }

    pub fn struct_to_json(data: MetaData) -> Result<String> {
        // Converting the MetaData structure to a JSON string
        Ok(serde_json::to_string(&data)?)
    }

    pub fn json_to_struct(data: String) -> Result<MetaData> {
        // Converting a JSON string to the MetaData structure
        Ok(serde_json::from_str(&data)?)
    }
}