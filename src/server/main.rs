extern crate env_logger;
use actix_cors::Cors;
use actix_web::{middleware, http, get, post, web::{self, Json, Path}, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use rand::Rng;
use actix_files::Files;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/json")]
async fn json(info: Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}", info.username))
}

#[get("/scramble")]
async fn scramble() -> impl Responder {
    // let mut rng = rand::thread_rng();
    // let shuffle: String = rng.gen_range(0, 99).to_string();
    let shuffle: String = "U R F D L B".to_string();

    HttpResponse::Ok().body(shuffle)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    print_launch_resume();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish())
            .service(scramble)
            .service(json)
            .service(Files::new("/", "./src/server/static/root/").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub fn print_launch_resume() {
    let ascii_name = r#" *******           **      ** **     **        
/**////**         /**     // /**    //*        
/**   /**  **   **/**      **/**  ** /   ******
/*******  /**  /**/****** /**/** **     **//// 
/**///**  /**  /**/**///**/**/****     //***** 
/**  //** /**  /**/**  /**/**/**/**     /////**
/**   //**//******/****** /**/**//**    ****** 
//     //  ////// /////   // //  //    //////  "#;
    eprintln!("{}", ascii_name);
    eprintln!();
    eprintln!("Server listening on:\t{:?}", "0.0.0.0:8080");
    eprintln!();
    eprintln!("Source code:\t\thttps://github.com/sgalasso42/rubik");
    eprintln!("Contact:\t\thttps://github.com/sgalasso42/rubik or bonjour@rubik.com");
    eprintln!();
}