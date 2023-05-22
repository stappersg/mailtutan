use crate::models::Message;
use std::sync::Mutex;
use tokio::sync::broadcast::Sender;

pub trait Storage: Sync + Send {
    fn list(&self) -> &Vec<Message>;
    fn add(&mut self, message: Message) -> usize;
    fn get(&self, item: usize) -> &Message;
    fn size(&self) -> usize;
    fn delete_all(&mut self);
}

// #[derive(Clone)]
pub struct Connection {
    pub storage: Mutex<Box<dyn Storage + 'static>>,
    pub ws_sender: Sender<String>,
}

pub struct Memory {
    sequence_id: usize,
    records: Vec<Message>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            records: vec![],
            sequence_id: 1,
        }
    }
}

impl Storage for Memory {
    fn list(&self) -> &Vec<Message> {
        &self.records
    }

    fn add(&mut self, mut message: Message) -> usize {
        message.id = Some(self.sequence_id);

        self.sequence_id += 1;

        self.records.push(message);

        self.sequence_id
    }

    fn get(&self, item: usize) -> &Message {
        &self.records.get(item - 1).unwrap()
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.records.len()
    }

    fn delete_all(&mut self) {
        self.records.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use super::Storage;
    use crate::models::Message;
    use std::assert_eq;

    #[test]
    fn test_store() {
        let mut store = Memory::new();

        store.add(Message {
            ..Default::default()
        });

        assert_eq!(store.size(), 1);

        store.delete_all();

        assert_eq!(store.size(), 0);
    }
}
