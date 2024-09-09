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

use actix_web::{web, App, HttpServer, HttpResponse};
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
                .route("/mgmt/{call}", web::post().to(handle_mgmt_call))
        })
        .bind(&format!("{}:{}", ip, port))?
        .run()
        .await
    }
}

async fn handle_mgmt_call(call: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Received variable: {}", call))
}