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
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use log::info;
use rsa::RsaPrivateKey;

use crate::encryption::rsa::Rsa;
use crate::encryption::aes::Aes;
use crate::encoding::base64::Base64;
use crate::keys::Keys;

pub struct ManagementServer;
pub struct Listener;

impl ManagementServer {
    pub async fn start(ip: String, port: u64, username: String, password: String, private_key: RsaPrivateKey) -> std::io::Result<()> {
        // Management side webserver

        info!("Starting management webserver on: {}:{}", ip, port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(private_key.clone()))
                .app_data(web::Data::new(username.clone()))
                .app_data(web::Data::new(password.clone()))
                // The route to /mgmt/call is a static path
                // This path will however change with the adoption of profiles
                // Making it possible to obscure the directory from tools like dirbuster

                .route("/mgmt/{api}", web::post().to(Self::handle_mgmt_call))       // Handle management actions (like queueing jobs for beacons)
                .route("/key/{action}", web::get().to(Self::handle_key_exchange))   // Retrieving the public key of the teamserver for secure communications
                .route("/key/{action}", web::post().to(Self::handle_key_exchange))  // Retrieving the private key of the teamclient upon authentication using AES and RSA
        })
        .bind(&format!("{}:{}", ip, port))?
        .run()
        .await
    }

    async fn handle_mgmt_call(api: web::Path<String>, body: String, private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
        // To test the function use the following PS logic: Invoke-RestMethod -Uri "http://127.0.0.1:1337/mgmt/test" -Method Post
        // The management function covers everything from starting new HTTP listeners to issueing commands to beacons
        // This function works in a REST API fashion.

        // Command: (Encrypted AES data + AES nonce):(Encrypted AES key):(Signed AES key)


        match api.as_str() {
            // Queueing a job for a beacon to do
            // Using the Public key of the teamclient we verify the authenticity and integrity of a command
            
            "queue" => {
                if body.len() > 0 && body.contains(":") {
                    let msg: Vec<&str> = body.split(":").collect();

                    if msg.len() >= 3 {
                        let encrypted_data = Base64::decode(msg[0]).to_vec();
                        let encrypted_key = Base64::decode(msg[1]).to_vec();
                        let signature = Base64::decode(msg[2]);

                        // Loading the public key of the teamclient
                        if let Ok(pubkey_location) = Keys::get_key_location("public_key_teamclient") {
                            if let Ok(teamclient_public_key) = Rsa::load_public_key(&pubkey_location) {

                                if Rsa::verify(teamclient_public_key, encrypted_key.clone(), &signature) {
                                    info!("Teamclient sent job, verified signature of msg");

                                    if let Ok(decrypted_key) = Rsa::decrypt(private_key.get_ref().clone(), encrypted_key) {
                                        let decrypted_command = Aes::decrypt(Aes::transform(decrypted_key), encrypted_data);
                                        if let Ok(command) = std::str::from_utf8(&decrypted_command) {
                                            
                                            // Insert logic for pushing the command to the queue
                                            
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "listener" => {
                // API call for starting a listener

                
            },
            _ => return HttpResponse::InternalServerError().into(),
        };

        HttpResponse::InternalServerError().into()
    }

    async fn handle_key_exchange(action: web::Path<String>, body: String, username: web::Data<String>, password: web::Data<String>, private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
        // Initial connections are not secure, this is why this function exists.
        // This functions serves the public key of the teamserver to encrypt a message
        // Retrieving the teamclients private key (through authentication)

        match action.as_str() {
            // Retrieving the private key of the teamclient (through authentication and encryption)
            "auth" => {
                if body.len() > 0 && body.contains(":") {
                    let msg: Vec<&str> = body.split(":").collect();

                    if msg.len() >= 2 {

                        let encrypted_req = msg[0];
                        let encrypted_key = msg[1];

                        if let Ok(aes_key) = Rsa::decrypt(private_key.get_ref().clone(), Base64::decode(&encrypted_key)) {
                            let aes_key = Aes::transform(aes_key);
                            let decrypted_req_body = Aes::decrypt(aes_key, Base64::decode(&encrypted_req));

                            if let Ok(plaintext) = std::str::from_utf8(&decrypted_req_body) {
                                if plaintext.len() > 0 && plaintext.contains("&") {
                                    let msg: Vec<&str> = plaintext.split("&").collect();
                                    
                                    if msg.len() >= 2 {
                                        let req_username = msg[0];
                                        let req_password = msg[1];

                                        if req_username == format!("username={}", username.get_ref().clone()) 
                                        && req_password == format!("password={}", password.get_ref().clone()) 
                                        {
                                            if let Ok(private_key_teamclient) = Keys::get_key_location("private_key_teamclient") {
                                                if let Ok(teamclient_private_key) = Rsa::load_private_key(&private_key_teamclient) {
                                                    if let Ok(b64_private_key) = Rsa::private_key_to_string(teamclient_private_key) {

                                                        // The teamclient does not yet have a public key, for this reason
                                                        // We reuse the AES key that was sent to the teamserver
                                                        // This AES key cannot be decrypted in transmission.
                                                        // Namely because this AES key was encrypted using RSA
                                                        let encrypted_private_key = Aes::encrypt(aes_key, b64_private_key.as_bytes().to_vec());
                                                        return HttpResponse::Ok().body(Base64::encode(&encrypted_private_key));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            // Serving the Public key of the Teamserver
            "get" => {
                let public_key = Rsa::public_key_from_private_key(private_key.get_ref().clone());
                if let Ok(b64_public_key) = Rsa::public_key_to_string(public_key) {
                    return HttpResponse::Ok().body(b64_public_key);
                }
            },
            _ => return HttpResponse::InternalServerError().into(),
        };

        HttpResponse::InternalServerError().into()
    }
}

impl Listener {
    pub async fn start_http_listener(ip: String, port: u64) -> std::io::Result<()> {
        // Starting a new listener given a host and port
        // This can be used to start a listener on a different interface that's on the server
        // Note: Starting a HTTP Listener will cause the request to run indefinitely

        info!("Starting HTTP Listener on: {}:{}", ip, port);

        HttpServer::new(move || {
            App::new()
                // As of right now the /search/{variable} path is a static path
                // This however changes with the adoption of profiles
                // Profiles will allow for the customization of these paths
                // Tailored specifically to an engagement.

                .route("/search/{variable}", web::post().to(http_listener_handler))  // Used for Beacon to post information
                .route("/search/{variable}", web::get().to(http_listener_handler))   // Used by beacons to communicate metadata
        })
        .bind(&format!("{}:{}", ip, port))?
        .run()
        .await
    }
}

async fn http_listener_handler(req: HttpRequest, info: web::Path<String>) -> impl Responder {
    // The main logic for the HTTP communications with the beacon goes here.
    // This server will listen for incoming connections and dispatch commands when received from mgmt
    info!("Method is: {}", req.method());
    format!("Hello, {}!", info)
}