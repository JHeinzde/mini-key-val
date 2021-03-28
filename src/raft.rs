use crate::raft::Action::Heartbeat;

pub struct Log(Vec<Action>);

#[derive(Debug)]
pub enum Action {
    Heartbeat,
    SetAction { key: String, value: Vec<u8> },
    DeleteAction { key: String },
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
            Action::SetAction { value: val, key: ke } => match other {
                Action::SetAction { value, key } => value == val && key == ke,
                _ => false
            }
            Action::DeleteAction { key: ke } => match other {
                Action::DeleteAction { key } => key == ke,
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
            Action::SetAction { value, key } => Action::SetAction { key: key.clone(), value: value.clone() },
            Action::DeleteAction { key } => Action::DeleteAction { key: key.clone() }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_log_append() {
        let set_action = Action::SetAction { key: String::from("test"), value: Vec::new() };
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
