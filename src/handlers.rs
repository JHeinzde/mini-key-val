use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::sync::Mutex;

use rocket::{Data, State};
use rocket::response::status::NotFound;

pub struct Cache(pub HashMap<String, Vec<u8>>);


#[post("/insert/<uuid>", data = "<object>")]
pub fn insert_into_cache(uuid: String, object: Data, storage: State<Mutex<Cache>>) -> Result<(), Box<dyn Error>> {
    let mut body_stream = object.open();
    let mut value = Vec::new();
    body_stream.read_to_end(&mut value)?;

    let mut lock = storage.lock().unwrap();
    lock.0.insert(uuid, value);

    return Ok(());
}

#[get("/retrieve/<uuid>")]
pub fn get_cache_value(uuid: String, storage: State<Mutex<Cache>>) -> Result<Vec<u8>, NotFound<String>> {
    let cache = storage.lock().unwrap();
    if cache.0.contains_key(&uuid) {
        Ok(cache.0.get(&uuid).unwrap().clone())
    } else {
        Err(NotFound(String::from("This key was not found")))
    }
}

#[delete("/delete/<uuid>")]
pub fn delete_cache_value(uuid: String, storage: State<Mutex<Cache>>) -> Result<(), NotFound<String>> {
    let mut cache = storage.lock().unwrap();
    if cache.0.contains_key(&uuid) {
        cache.0.remove(&uuid);
        Ok(())
    } else {
        Err(NotFound(String::from("This key was not found")))
    }
}
