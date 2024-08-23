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


// Just like the TCP implementation, this implementation also is required to be bidirectional.
// The reasoning behind this is the exact same as with the TCP implementation;
// It will cause only one beacon to reach out to the C2 at a time (which reduces detectability)
// As of right now it's not bidirectional but unidirectional

pub struct Pipes;

impl Pipes {
    pub fn connect() {
        // Connecting to a pipe
        todo!();
    }

    pub fn peek() {
        // Peeking into a pipe, this is to say if there are bytes in the pipe
        todo!();
    }

    pub fn read() {
        // Reading the information out of the pipe
        todo!();
    }

    pub fn write() {
        // Writing information to the pipe
        todo!();
    }
}