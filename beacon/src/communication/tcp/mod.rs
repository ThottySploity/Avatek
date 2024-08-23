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
use std::net::TcpStream;

use std::io::{Read, Write};

pub struct Tcp;

// This impl still requires the Listener logic
// So any beacon can be turned into a pivot listener that bridges the gap between machines
// Without having multiple beacons reach out to the C2 from the same network.

// The functionality of the TCP needs to be bidirectional, currently it's unidirectional

impl Tcp {
    pub fn connect(host: String, port: String) -> Result<TcpStream> {
        // Functionality so that beacon can connect to a TCP server
        let stream = TcpStream::connect(&format!("{}:{}", host, port))?;
        Ok(stream)
    }

    pub fn read(mut stream: TcpStream) -> Result<Vec<u8>> {
        // Reading bytes from the TCP stream
        let mut buffer = Vec::new();
        stream.read(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write(mut stream: TcpStream, data: Vec<u8>) -> Result<()> {
        // Writing bytes to the TCP stream
        stream.write(&data)?;
        Ok(())
    }
}