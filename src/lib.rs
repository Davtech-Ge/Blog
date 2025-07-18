mod errors;
mod models;
mod routes;
mod schema;

extern crate diesel;

#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ self, ConnectionManager};


type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Blog {
    port: u16,
}

impl Blog {
    pub fn new(port: u16) -> Self {
        Blog { port }
    }

    pub async fn run(&self, database_url: String) -> std::io::Result<()> {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        println!("Starting server at port 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .configure(routes::users::configure)
                .configure(routes::posts::configure)
                .configure(routes::comments::configure)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}