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

use crate::webserver::LISTENERS;

use actix_web::dev::ServerHandle;
use log::debug;

pub struct ListenerUtils;

impl ListenerUtils {
    
    pub fn add_listener_to_queue(listener_type: String, host: String, port: String, listener_handle: ServerHandle) {
        // Adding a listener to the LISTENERS variable
        // This is to keep track of what listeners are on the system

        if let Ok(mut listeners) = LISTENERS.lock() {
            debug!("Adding {} listener to teamserver on: {}:{}", listener_type, host, port);
            listeners.listeners.push((listener_type, host, port, listener_handle));
            debug!("Number of listeners: {}", listeners.listeners.len());
        }
    }

    pub async fn remove_listener_from_queue(listener_type: String, host: String, port: String) -> bool {
        // Removing an unused listener
        if let Ok(mut listeners) = LISTENERS.lock() {
            for (index, (active_listener_type, active_host, active_port, listener_handle)) in listeners.listeners.clone().iter().enumerate() {
                if *active_listener_type == listener_type && *active_host == host && *active_port == port {
                    
                    // First since it does exist, we need to stop the listener
                    listener_handle.stop(true).await; // Gracefull stop of the listener
                    debug!("Removed: {}:{}:{} from active listeners", listener_type, host, port);
                    listeners.listeners.remove(index);
                    return true;
                }
            }
        }
        false
    }

    pub fn check_listener_in_queue(listener_type: String, host: String, port: String) -> bool {
        // Checking to make sure we don't have the same listener twice on the same system
        if let Ok(listeners) = LISTENERS.lock() {
            for (_, (active_listener_type, active_host, active_port, _)) in listeners.listeners.iter().enumerate() {
                if *active_listener_type == listener_type && *active_host == host && *active_port == port {
                    return true;
                }
            }
        }
        false
    }
}