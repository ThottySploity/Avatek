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

use crate::utilities::beacondecoder::BeaconDecoder;
use crate::webserver::{QUEUE, endpoints::*, endpoints::listeners::Listeners};

use log::{debug, error, info};
use rsa::RsaPrivateKey;
use actix_web::{web, HttpResponse, HttpServer, App};

pub struct ManagementServer;

impl ManagementServer {
    pub async fn start(ip: String, port: u64, username: String, password: String, private_key: RsaPrivateKey) -> std::io::Result<()> {
        // Management side webserver

        info!("Starting 2 management webserver on: {}:{}", ip, port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(private_key.clone()))
                .app_data(web::Data::new(username.clone()))
                .app_data(web::Data::new(password.clone()))

                // The route to /mgmt/ and /key/ are static paths
                // With the introduction of profiles, these will be changable

                .route("/mgmt/{api}", web::post().to(handle_mgmt_call))         // Handle management actions (like queueing jobs for beacons)
                .route("/key/{action}", web::get().to(handle_key_exchange))     // Retrieving the public key of the teamserver for secure communications
                .route("/key/{action}", web::post().to(handle_key_exchange))    // Retrieving the private key of the teamclient upon authentication using AES and RSA
        })
        .bind(&format!("{}:{}", ip, port))?
        .run()
        .await
    }
}

async fn handle_mgmt_call(api: web::Path<String>, body: String, private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
    // The management function covers everything from starting new HTTP listeners to issueing commands to beacons
    // This function works in a REST API fashion.

    // Requests to this API endpoint follow the following structure:
    // (Encrypted AES data):(Encrypted AES key):(Signed AES key)

    match api.as_str() {
        // Queueing a job for a beacon to do
        // Using the Public key of the teamclient we verify the authenticity and integrity of a command
        
        "queue" => {
            // Firstly verify the message is from the teamclient
            if let Err(e) = verify_message(body.clone(), "public_key_teamclient") {
                error!("{} for queue endpoint", e);
                return HttpResponse::NotFound().into();
            }

            // Retrieving the command from the HTTP body
            if let Ok(command) = retrieve_command(body.clone(), private_key.get_ref().clone()) {
                if let Ok(mut queue) = QUEUE.lock() {
                    let beacon = BeaconDecoder::new(command.to_string());
                    debug!("Added a command for beacon: {}", beacon.id());
                    queue.commands.push((beacon.id(), beacon.res()));
                    debug!("Length of queue: {}", queue.commands.len());
                }
            }
        },
        "listener" => {
            // Firstly verify the message is from the teamclient
            if let Err(e) = verify_message(body.clone(), "public_key_teamclient") {
                error!("{} for listener endpoint", e);
                return HttpResponse::NotFound().into();
            }

            // Parse the resulting command to the Listener impl
            if let Ok(command) = retrieve_command(body.clone(), private_key.get_ref().clone()) {
                let _ = Listeners::handle_listener_call(command).await;
            }
        },
        _ => return HttpResponse::NotFound().into(),
    };
    HttpResponse::NotFound().into()
}

async fn handle_key_exchange(action: web::Path<String>, body: String, username: web::Data<String>, password: web::Data<String>, private_key: web::Data<RsaPrivateKey>) -> HttpResponse {
    // Initial connections are not secure, this is why this function exists.
    // This functions serves the public key of the teamserver to encrypt a message
    // Retrieving the teamclients private key (through authentication)

    match action.as_str() {
        "auth" => {
            // Retrieving the private key of the teamclient (through authentication and encryption)
            // Ideally you don't want to send the password over in any case, encrypted or not.
            // You'd rather want to do a Challenge-Response

            if let Ok(encrypted_private_key) = get_encrypted_private_key(
                body, 
                username.get_ref().clone(), 
                password.get_ref().clone(), 
                private_key.get_ref().clone()
            ) {
                return HttpResponse::Ok().body(Base64::encode(&encrypted_private_key));
            }
        },
        "get" => {
            // Serving the public key of the teamserver to encrypt data that only the teamserver can read
            let teamserver_public_key = Rsa::public_key_from_private_key(private_key.get_ref().clone());
            if let Ok(b64_public_key) = Rsa::public_key_to_string(teamserver_public_key) {
                return HttpResponse::Ok().body(b64_public_key);
            }
        },
        _ => return HttpResponse::NotFound().into(),
    };

    
    HttpResponse::NotFound().into()
}

fn get_encrypted_private_key(body: String, username: String, password: String, private_key: RsaPrivateKey) -> Result<Vec<u8>> {

    // This function retrieves the private_key of the teamclient that was generated by the teamserver
    // To exchange this, the teamclient has to authenticate with the username and password of the teamserver (this was chosen at startup of the teamserver)
    // The data is then transmitted using AES-GCM 

    if body.len() > 0 && body.contains(":") {
        let msg: Vec<&str> = body.split(":").collect();

        if msg.len() >= 2 {

            let encrypted_req = Base64::decode(&msg[0]);
            let encrypted_key = Base64::decode(&msg[1]);

            let decrypted_aes_key = Rsa::decrypt(private_key, encrypted_key)?;
            let aes_key = Aes::transform(decrypted_aes_key);
            let decrypted_req = Aes::decrypt(aes_key, encrypted_req);

            let plaintext = Utils::convert(&decrypted_req)?.to_string();
            if plaintext.len() > 0 && plaintext.contains("&") {
                let msg: Vec<&str> = plaintext.split("&").collect();

                if msg.len() >= 2 {
                    let req_username = msg[0];
                    let req_password = msg[1];

                    if req_username == format!("username={}", username) 
                    && req_password == format!("password={}", password) 
                    {
                        let private_key_teamclient = Rsa::load_private_key("private_key_teamclient")?;
                        let b64_private_key = Rsa::private_key_to_string(private_key_teamclient)?;

                        // The teamclient does not yet have a public key, for this reason
                        // We reuse the AES key that was sent to the teamserver
                        // This AES key cannot be decrypted in transmission.
                        // Namely because this AES key was encrypted using RSA
                        let encrypted_private_key = Aes::encrypt(aes_key, b64_private_key.as_bytes().to_vec());
                        return Ok(encrypted_private_key);
                    }
                }
            }
        }
    }

    Err(anyhow!(""))
}