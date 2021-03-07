extern crate env_logger;
use actix_cors::Cors;
use actix_web::{middleware, http, get, post, web::{Json}, App,  HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use actix_files::Files;

use rubik_lib::rubik::cubie_cube::{CubieCube};
use rubik_lib::pruning::pruning::{Pruning};
use rubik_lib::algo::solve::*;
use rubik_lib::rubik::enums::{ACTIONS_STR_LIST};

mod parsing;

use parsing::parse::{parse_inputs};

#[derive(Deserialize)]
struct Request {
    sequence: String,
}

#[derive(Serialize)]
struct Response {
    solution: String,
}

#[get("/scramble")]
async fn scramble() -> impl Responder {
    let shuffle: String = "U R F D L B".to_string();
    HttpResponse::Ok().body(shuffle)
}

#[post("/solver")]
async fn solver(req: Json<Request>) -> impl Responder {
    let pruning_tables: Pruning = Pruning::new();
    let mut cb_cube: CubieCube = CubieCube::new_solved();
    let input_sequence: Vec<usize> = parse_inputs(&req.sequence);

    cb_cube.apply_sequence(&input_sequence);
    let mut solution: Vec<usize> = Vec::new();
    match solve(&mut cb_cube, &pruning_tables) {
        Some(s) => 
            return HttpResponse::Ok().json(Response {
            solution: s.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" ").to_owned(),
        }),
        None => return HttpResponse::Ok().json(Response {
            solution: "".to_string(),
        })
    }
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
            .service(solver)
            .service(Files::new("/", "./public").index_file("index.html"))
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