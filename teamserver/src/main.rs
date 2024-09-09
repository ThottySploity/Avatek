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
pub mod encryption;
pub mod keys;
pub mod utilities;
pub mod webserver;

use encryption::rsa::Rsa;
use keys::Keys;
use webserver::Webserver;

use env_logger::Env;
use clap::Parser;

#[derive(Parser)]
#[clap(about, author)]
pub struct Args {

    /// Management side IP address of the teamserver
    #[clap(short, long, default_value = "127.0.0.1")]
    teamserver_ip_address: String,

    /// Management side Port of the teamserver
    #[clap(short, long, default_value = "1337")]
    port: u64,

    /// Location of the profile.toml file
    #[clap(long, default_value = "")]
    profile: String,

    /// Create keypair for Avatek
    #[clap(short, long)]
    generate: bool,

    /// Force an overwrite of any found keys
    #[clap(short)]
    force: bool,
}

#[actix::main]
async fn main() {
    let args = Args::parse();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    
    if !Keys::exists() || args.generate {
        // This function will generate the keypairs for;
        //  • teamclient
        //  • teamserver
        //  • beacons

        // This function will be called if either the keys do not exist yet
        // Or if an operator overwrites the current keys
        Keys::generate(args.force);
    }

    if let Ok(private_key_location) = Keys::get_key_location("private_key_beacon") {
        if let Ok(private_key) = Rsa::load_private_key(&private_key_location) {

            // Import the keys for the Teamserver to communicate
            let _ = Webserver::start(args.teamserver_ip_address, args.port, private_key).await;
        }
    }
}
