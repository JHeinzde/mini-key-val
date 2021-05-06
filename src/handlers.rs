use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::sync::Mutex;

use rocket::{Data, State};
use rocket::response::status::{NotFound, BadRequest};
use crate::raft::{RaftState, AppendEntry};
use rocket_contrib::json::Json;

pub struct Cache(pub HashMap<String, Vec<u8>>);


#[post("/insert/<uuid>", data = "<object>")]
pub fn insert_into_cache(uuid: String, object: Data, storage: State<Mutex<Cache>>, raft: State<Mutex<RaftState>>) -> Result<(), Box<dyn Error>> {
    let mut body_stream = object.open();
    let mut value = Vec::new();
    body_stream.read_to_end(&mut value)?;

    let mut lock = storage.lock().unwrap();
    lock.0.insert(uuid, value);

    return Ok(());
}

#[get("/retrieve/<uuid>")]
pub fn get_cache_value(uuid: String, storage: State<Mutex<Cache>>, raft: State<Mutex<RaftState>>) -> Result<Vec<u8>, NotFound<String>> {
    let cache = storage.lock().unwrap();
    if cache.0.contains_key(&uuid) {
        Ok(cache.0.get(&uuid).unwrap().clone())
    } else {
        Err(NotFound(String::from("This key was not found")))
    }
}

#[delete("/delete/<uuid>")]
pub fn delete_cache_value(uuid: String, storage: State<Mutex<Cache>>, raft: State<Mutex<RaftState>>) -> Result<(), NotFound<String>> {
    let mut cache = storage.lock().unwrap();
    if cache.0.contains_key(&uuid) {
        cache.0.remove(&uuid);
        Ok(())
    } else {
        Err(NotFound(String::from("This key was not found")))
    }
}

#[post("/append_entries", data = "<entry>")]
pub fn append_entries(entry: Json<AppendEntry>, raft: State<Mutex<RaftState>>) -> Result<(), BadRequest<String>> {
    let mut raft_state = raft.lock().unwrap();

    if entry.0.term < raft_state.current_term {
        return Err(BadRequest(Option::from(String::from("Term sent is smaller than current_term"))))
    }


    Ok(())
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    use super::*;

    #[test]
    fn test_insert() {
        let storage: HashMap<String, Vec<u8>> = HashMap::new();
        let cache = Cache(storage);
        let rocket = rocket::ignite().mount("/", routes![get_cache_value,
            insert_into_cache,
            delete_cache_value])
            .manage(Mutex::new(cache));
        let client = Client::new(rocket).expect("valid rocket instance");

        let key = String::from("this-is-a-test-key");
        let mut value: Vec<u8> = Vec::new();
        value.push(32);
        value.push(64);
        value.push(128);

        let req = client.post(["/insert/", &key].join(""))
            .body(value.clone());
        let response = req.dispatch();

        assert_eq!(Status::Ok, response.status());

        let req = client.get(["/retrieve/", &key].join(""));
        let mut response = req.dispatch();

        assert_eq!(Status::Ok, response.status());
        assert_eq!(value.clone(), response.body_bytes().unwrap())
    }

    #[test]
    fn test_delete() {
        let storage: HashMap<String, Vec<u8>> = HashMap::new();
        let cache = Cache(storage);
        let rocket = rocket::ignite().mount("/", routes![get_cache_value,
            insert_into_cache,
            delete_cache_value])
            .manage(Mutex::new(cache));
        let client = Client::new(rocket).expect("valid rocket instance");

        let key = String::from("this-is-a-test-key");
        let mut value: Vec<u8> = Vec::new();
        value.push(32);
        value.push(64);
        value.push(128);

        let req = client.post(["/insert/", &key].join(""))
            .body(value.clone());
        let _response = req.dispatch();

        let req = client.delete(["/delete/", &key].join(""));
        let response = req.dispatch();

        assert_eq!(Status::Ok, response.status());

        let req = client.get(["/retrieve/", &key].join(""));
        let response = req.dispatch();

        assert_eq!(Status::NotFound, response.status());
    }
}
