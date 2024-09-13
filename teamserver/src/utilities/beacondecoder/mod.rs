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

pub struct BeaconDecoder {
    id: String,
    req: String,
}

impl BeaconDecoder {
    pub fn new(body: String) -> Self {
        
        if body.contains(":") {
            let msg: Vec<&str> = body.split(":").collect();
            
            if msg.len() >= 2 {

                let id = msg[0];
                let req = msg[1];

                return Self {
                    id: id.to_string(),
                    req: req.to_string(),
                }
            }
        }

        // This function should not be called (if the teamclient is used as intended)
        Self {
            id: "".to_string(),
            req: "".to_string(),
        }
    }

    pub fn id(&self) -> String {
        // Getting the Beacon ID
        self.id.clone()
    }

    pub fn res(&self) -> String {
        // Getting the Beacon String
        self.req.clone()
    }
}