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

pub mod endpoints;

use actix_web::dev::ServerHandle;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct ManagementServer;
pub struct Listener;

// Define a structure to hold the stored data
#[derive(Default)]
struct Commands {
    commands: Vec<(String, String)>
}

// Define a structure to hold the active listeners (type, ip, port, handle)
#[derive(Default)]
struct Listeners {
    listeners: Vec<(String, String, String, ServerHandle)>
}

#[derive(Default)]
pub struct Teamclients {
    clients: Vec<Teamclient>
}

// Define a structure to hold the information about the teamclient
// As the teamclient is dynamic, it will be added once it has authenticated against the teamserver.
// The teamserver will use this information to proxy Beacon POST requests to the teamclient
// Here the teamclient can then show an operator the results of an executed command on a target
#[derive(Default)]
pub struct Teamclient {
    host: String,
    port: String,
    endpoint: String,
}

impl Teamclient {
    pub fn new(host: String, port: String, endpoint: String) -> Self {
        Self {host, port, endpoint}
    }

    pub fn url(&self) -> String {
        format!("{}:{}/{}", self.host, self.port, self.endpoint)
    }
}

// Initialize a lazy_static Mutex to store queued commands and listeners
lazy_static! {
    static ref QUEUE: Mutex<Commands> = Mutex::new(Commands::default());
    static ref LISTENERS: Mutex<Listeners> = Mutex::new(Listeners::default());
    static ref TEAMCLIENT: Mutex<Teamclients> = Mutex::new(Teamclients::default());
}