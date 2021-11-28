extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
use dotenv::dotenv;
use std::env;
use routes::*;
mod db;
mod models;
mod routes;
mod schema;
mod static_files;
fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("database_rul").expect("set the database_url");
    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author, update],
        )
        .mount("/", routes![static_files::all, static_files::index])
}

fn main() {
    rocket().launch();
}
