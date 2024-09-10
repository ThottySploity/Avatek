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

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use log::info;
use rsa::RsaPrivateKey;

pub struct Webserver;

impl Webserver {
    pub async fn start(ip: String, port: u64, private_key: RsaPrivateKey) -> std::io::Result<()> {
        // Management side webserver

        info!("Starting management webserver on: {}:{}", ip, port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(private_key.clone()))
                // The route to /mgmt/call is a static path
                // This path will however change with the adoption of profiles
                // Making it possible to obscure the directory from tools like dirbuster

                .route("/mgmt/{call}", web::post().to(handle_mgmt_call))
        })
        .bind(&format!("{}:{}", ip, port))?
        .run()
        .await
    }
}

async fn handle_new_http_listener(ip: web::Path<String>, port: web::Path<u64>) -> std::io::Result<()> {
    // Starting a new listener given a host and port
    // This can be used to start a listener on a different interface that's on the server

    info!("Starting new listener on: {}:{}", ip, port);

    HttpServer::new(move || {
        App::new()
            // As of right now the /search/{variable} path is a static path
            // This however changes with the adoption of profiles
            // Profiles will allow for the customization of these paths
            // Tailored specifically to an engagement.
            
            .route("/search/{variable}", web::post().to(handle_http_listener))
    })
    .bind(&format!("{}:{}", ip, port))?
    .run()
    .await
}

async fn handle_http_listener(info: web::Path<String>) -> impl Responder {
    // The main logic for the HTTP communications with the beacon goes here.
    // This server will listen for incoming connections and dispatch commands when received from mgmt

    info!("Received request for variable: {}", info);
    format!("Hello, {}!", info)
}

async fn handle_mgmt_call(call: web::Path<String>) -> HttpResponse {
    // The management function covers everything from starting new HTTP listeners to issueing commands to beacons
    // All data in transit is encrypted like the following:
    // Beacon <- ENCRYPTED DATA -> Teamserver <- ENCRYPTED DATA -> Teamclient

    HttpResponse::Ok().body(format!("Received variable: {}", call))
}