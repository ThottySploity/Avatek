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

pub mod communication;
pub mod runtimes;
pub mod configuration;
pub mod functionality;
pub mod utilities;

use clap::Parser;

use runtimes::{svc::RuntimeSvc, exe::RuntimeExe};

#[derive(Parser)]
struct Args {
    #[clap(short)]
    service: bool,
}

fn main() {
    let args = Args::parse();

    if args.service {
        // A service is a bit different than a regular exe file
        // As we have this boolean from command-line, we need one code base
        // That can both run as a standalone EXE aswell as run as a Service
        // Pretty neat right :)

        RuntimeSvc::new("Service".to_string());
    }
    RuntimeExe::new();
}