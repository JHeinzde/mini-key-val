use crate::raft::Action::Heartbeat;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Log(Vec<Action>);

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Heartbeat,
    SetAction { key: String, value: Vec<u8>, term: u32 },
    DeleteAction { key: String, term: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaftState {
    pub current_term: u32,
    pub voted_for: u32,
    pub log: Log,
    pub volatile_state: VolatileState,
    pub leader_state: LeaderState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolatileState {
    pub commit_index: u64,
    pub last_applied: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderState {
    pub next_index: Vec<u64>,
    pub match_index: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppendEntry {
    pub term: u32,
    pub leader_id: u32,
    pub prev_log_index: u64,
    pub prev_log_term: u32,
    pub entries: Vec<Action>,
    pub leader_commit: u64,
}

impl Log {
    pub fn append(&mut self, ac: Action) {
        match ac {
            Action::Heartbeat => (),
            _ => {
                self.0.push(ac);
            }
        }
    }

    pub fn get_last_action(self) -> Option<Action> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.last().unwrap().clone())
        }
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Action::Heartbeat => match other {
                Action::Heartbeat => true,
                _ => false
            }
            Action::SetAction { value: val, key: ke, term: te } => match other {
                Action::SetAction { value, key, term } => value == val && key == ke && term == te,
                _ => false
            }
            Action::DeleteAction { key: ke, term: te } => match other {
                Action::DeleteAction { key, term } => key == ke && term == te,
                _ => false
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


impl Clone for Action {
    fn clone(&self) -> Self {
        match self {
            Action::Heartbeat => Heartbeat,
            Action::SetAction { value, key, term } =>
                Action::SetAction { key: key.clone(), value: value.clone(), term: term.clone() },
            Action::DeleteAction { key, term } => Action::DeleteAction { key: key.clone(), term: term.clone() }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_log_append() {
        let set_action = Action::SetAction { key: String::from("test"), value: Vec::new(), term: 32 };
        let mut log_under_test = Log(Vec::new());

        log_under_test.append(set_action.clone());
        assert_eq!(set_action, log_under_test.get_last_action().unwrap())
    }

    #[test]
    fn test_empty_lgo() {
        let log_under_test = Log(Vec::new());

        assert_eq!(None, log_under_test.get_last_action())
    }
}
