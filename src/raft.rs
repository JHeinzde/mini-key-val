
pub struct Log(Vec<Action>);

pub enum Action {
    HEARTBEAT,
    SET_ACTION(key: String, value: Vec<u8>),
    DELETE_ACTION(key: String)
}
