#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::{mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use crate::handlers::Cache;
use crate::raft::Action;

mod handlers;
mod raft;

fn main() {
    let storage: HashMap<String, Vec<u8>> = HashMap::new();
    let cache = Cache(storage);
    let sender = setup_raft_thread();

    rocket::ignite().mount("/", routes![handlers::get_cache_value,
     handlers::insert_into_cache,
     handlers::delete_cache_value])
        .manage(Mutex::new(cache))
        .manage(Mutex::new(sender))
        .launch();
}


fn setup_raft_thread() -> Sender<Action> {
    let (sender, receiver) = mpsc::channel();
    let _raft = std::thread::spawn(move || {
        loop {
            println!("Kekw");
            println!("{:?}", receiver.recv().unwrap());
        }
    });
    return sender;
}
