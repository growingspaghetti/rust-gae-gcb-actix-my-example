use actix_web::{error, middleware, post, put, web, App, HttpServer, Responder};
use bytes::BytesMut;
use env_logger;
use futures::StreamExt;
use log::info;
use std::fs::File;
use std::io::prelude::*;

const MAX_SIZE: usize = 262_144_000; // max payload size is 256M

#[post("/ee18a22f-f18a-4f71-bb59-f507473e53f4/{path}/{file}")]
async fn post1(info: web::Path<(String, String)>, mut payload: web::Payload) -> impl Responder {
    println!("path: {}, file: {}", info.0, info.1);
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let mut file = File::create(&info.1)?;
    file.write_all(&body[..])?;
    Ok(format!("file created:{}", &info.1))
}

#[put("/ee18a22f-f18a-4f71-bb59-f507473e53f4/{path}/{file}")]
async fn put1(info: web::Path<(String, String)>, mut payload: web::Payload) -> impl Responder {
    println!("path: {}, file: {}", info.0, info.1);
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let mut file = File::create(&info.1)?;
    file.write_all(&body[..])?;
    Ok(format!("file created:{}", &info.1))
}

#[post("/ee18a22f-f18a-4f71-bb59-f507473e53f4/{file}")]
async fn post2(info: web::Path<String>, mut payload: web::Payload) -> impl Responder {
    println!("file: {}", info);
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let mut file = File::create(&info.as_str())?;
    file.write_all(&body[..])?;
    Ok(format!("file created:{}", &info))
}

#[put("/ee18a22f-f18a-4f71-bb59-f507473e53f4/{file}")]
async fn put2(info: web::Path<String>, mut payload: web::Payload) -> impl Responder {
    println!("file: {}", info);
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let mut file = File::create(&info.as_str())?;
    file.write_all(&body[..])?;
    Ok(format!("file created:{}", &info))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(put1)
            .service(put2)
            .service(post1)
            .service(post2)
            .service(
                actix_files::Files::new("/ee18a22f-f18a-4f71-bb59-f507473e53f4", ".")
                    .show_files_listing(),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
