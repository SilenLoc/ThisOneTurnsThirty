#![deny(future_incompatible, nonstandard_style, unsafe_code, rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::single_match_else,
    clippy::let_unit_value,
    clippy::no_effect_underscore_binding,
    clippy::needless_pass_by_value,
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::items_after_test_module
)]

#[macro_use]
extern crate rocket;
#[cfg(test)]
#[macro_use]
extern crate rstest;

mod auth;
mod sql;
mod api;

use std::env;

use auth::AdminAccessToken;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{routes, Request, Response};
use sql::init;

#[get("/healthz")]
fn healthz() {}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let db = sql::init().await.expect("DB is needed to start");
    let admin_access_token = env::var("ADMIN_ACCESS_TOKEN").expect("needs to have a admin token");
    let rocket = rocket::build()
        .mount("/api", routes![healthz])
        .mount("/", routes![all_options])
        .manage(db)
        .manage(AdminAccessToken{code: admin_access_token})
        .attach(CORS);

    Ok(rocket.into())
}

#[cfg(test)]
mod tests {
    use rocket::{http::Status, local::blocking::Client};

    use super::*;

    #[fixture]
    fn client() -> Client {
        Client::tracked(
            rocket::build()
                .mount("/api", routes![healthz])
                .mount("/", routes![all_options]),
        )
        .unwrap()
    }

    #[rstest]
    fn healthz_returns_ok(client: Client) {
        assert_eq!(Status::Ok, client.get("/api/healthz").dispatch().status());
    }
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
