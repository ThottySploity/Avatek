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

pub mod utils;
use crate::webserver::endpoints::listeners::utils::ListenerUtils;

use actix_web::{web, rt, HttpServer, App, Responder, HttpRequest};

use anyhow::{anyhow, Result};
use log::{info};

pub struct Listeners;

impl Listeners {
    pub async fn handle_listener_call(info: String) -> Result<()> {
        // The message was already verified in the mgmt endpoint

        let msg: Vec<&str> = info.split(":").collect();

        if msg.len() >= 4 {
            let listener_meth = msg[0].to_string();
            let listener_type = msg[1].to_string();
            let listener_host = msg[2].to_string();
            let listener_port = msg[3].to_string();

            // Example msg:
            // add:http:127.0.0.1:1338
            // remove:http:127.0.0.1:1338

            match listener_meth.as_str() {
                "add" => {
                    match listener_type.as_str() {
                        "http" => {
                            if !ListenerUtils::check_listener_in_queue(listener_type.clone(), listener_host.clone(), listener_port.clone()) {
                                // Listener is not added yet
                                let _ = start_http_listener(listener_host, listener_port).await;
                            }
                        },
                        _ => return Err(anyhow!("Listener type does not exist")),
                    }
                },
                "remove" => {
                    // First check it's there
                    if ListenerUtils::check_listener_in_queue(listener_type.clone(), listener_host.clone(), listener_port.clone()) {
                        // Removing the server from the global variable
                        // This will also stop the running server
                        ListenerUtils::remove_listener_from_queue(listener_type.clone(), listener_host.clone(), listener_port.clone()).await;
                    }
                },
                "get" => {
                    // Get all the active listeners
                    // if let Ok(listeners) = LISTENERS.lock() {
                    //     for (index, (_, _, _, _)) in listeners.listeners.clone().iter().enumerate() {
                    //         // I should turn the output of this into JSON 
                    //         // So we have a JSON format of all the listeners. Something like this:

                    //         // {
                    //         //     "Listeners": [
                    //         //         {
                    //         //             "index": "0",
                    //         //             "type": "http",
                    //         //             "host": "127.0.0.1",
                    //         //             "port": "1337"
                    //         //         },
                    //         //         {
                    //         //             "index": "1",
                    //         //             "type": "http",
                    //         //             "host": "127.0.0.1",
                    //         //             "port": "1338"
                    //         //         }
                    //         //     ]
                    //         // }

                    //         todo!();
                    //     }
                    // }
                    todo!();
                }
                _ => return Err(anyhow!("Method does not exist")),
            }
        }
        Err(anyhow!("Invalid listener call"))
    }
}

async fn start_http_listener(host: String, port: String) -> std::io::Result<()> {
    // Starting a new listener given a host and port
    // This can be used to start a listener on a different interface that's on the server

    info!("Starting HTTP Listener on: {}:{}", host, port);
    
    let listener = HttpServer::new(move || {
        App::new()
            // With the introduction of Profiles these /search/ paths will be changable.

            .route("/search/{variable}", web::post().to(http_listener_handler))  // Used for Beacon to post information
            .route("/search/{variable}", web::get().to(http_listener_handler))   // Used by beacons to communicate metadata 
    })
    .bind(&format!("{}:{}", host, port))?
    .run();
    
    let listener_handle = listener.handle();

    // Add the listener to a global variable so we can check cross threads
    ListenerUtils::add_listener_to_queue("http".to_string(), host, port, listener_handle);
    rt::spawn(listener);
    Ok(())
}

async fn http_listener_handler(req: HttpRequest, info: web::Path<String>) -> impl Responder {
    // The main logic for the HTTP communications with the beacon goes here.
    // This server will listen for incoming connections and dispatch commands when received from mgmt
    info!("Method is: {}", req.method());
    format!("Hello, {}!", info)
}