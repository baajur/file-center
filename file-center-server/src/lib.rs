#![allow(dead_code)]
extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate jsonwebtoken;
extern crate log;
extern crate serde_json;
extern crate sqlx;
extern crate strum;
extern crate uuid;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate anyhow;

pub mod config;
pub mod errors;
pub mod handlers;
pub mod middlewares;
pub mod models;
pub mod payloads;
pub mod routers;
pub mod services;
pub mod utils;

pub use models::DataBase;