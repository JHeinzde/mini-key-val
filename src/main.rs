#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::Mutex;

use crate::handlers::Cache;

mod handlers;
mod raft;

fn main() {
    let storage: HashMap<String, Vec<u8>> = HashMap::new();
    let cache = Cache(storage);
    rocket::ignite().mount("/", routes![handlers::get_cache_value,
     handlers::insert_into_cache,
     handlers::delete_cache_value])
        .manage(Mutex::new(cache))
        .launch();
}

