use std::time::Duration;

use actix_web::{get, http::header, App, HttpResponse, HttpServer};
use bytes::Bytes;

#[get("/")]
async fn index() -> HttpResponse {
    println!("i am in");
    std::thread::sleep(Duration::from_secs(5));
    HttpResponse::Ok().body("Hello world!")
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let (mut tx, rx) = futures::channel::mpsc::channel::<Result<Bytes, anyhow::Error>>(1);
    let _guard = actix_web::rt::spawn(async move {
        // long-running writes to the channel
        for i in 1..=10 {
            let msg = format!("{}\n", i);
            println!("sending: {}", msg);
            if let Err(e) = tx.try_send(Ok(Bytes::from(msg))) {
                println!("error: {}", e);
                println!("connection dropped, stopping the actor");
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::TEXT_EVENT_STREAM))
        .streaming(rx)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(stream))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
