#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate itconfig;

use rocket::config::{Config, Environment};

config! {
    ROCKET {
        HOST: String => "localhost",
        PORT: u16 => 9000,
        BASE_URL: String => "/",
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    cfg::init();

    rocket::ignite()
        .mount(&cfg::ROCKET::BASE_URL(), routes![index])
        .launch();
}