use core::time;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
// use std::{error::Error, thread::current};
use std::thread;

pub enum ID {
    Id(u64),
    Time(u64),
    MachineID(u64),
}

// Sarmio is a distributed unique ID generator
// It has 4  methods that you can use
// new
// next_id
// next
// machine_id
// get_machine_id
//
// machine id must be smaller than 256.
pub struct Sarmio {
    time: Arc<Mutex<u64>>,
    machine_id: u64,
    sequence: u64,
}

impl Sarmio {
    pub fn new(machine_id: u64) -> Sarmio {
        Sarmio {
            time: Arc::new(Mutex::new(0)),
            machine_id: machine_id,
            sequence: 0,
        }
    }

    // Creates a new unique ID.
    pub fn next_id(&mut self) -> u64 {
        let mut current_time = self.time.lock().unwrap();
        let mut timestamp = self.get_time();
        if timestamp < *current_time {
            sleep((*current_time - timestamp) + 1);
            timestamp = self.get_time();
        } else if timestamp == *current_time {
            self.sequence = (self.sequence + 1) & self.machine_id
        } else {
            self.sequence = 0;
        }
        *current_time = timestamp;
        (timestamp << 24) | self.sequence << 16 | self.machine_id
    }

    // Set machine ID
    pub fn machine_id(&mut self, id: u64) {
        self.machine_id = id;
    }

    pub fn get_machine_id(&self) -> u64 {
        self.machine_id
    }

    fn get_time(&self) -> u64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(t) => t.as_secs(),
            Err(_) => panic!("system time before unix epoch"),
        }
    }
}

fn sleep(milis: u64) {
    thread::sleep(time::Duration::from_millis(milis));
}

pub fn decompose(id: u64) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    let mask_machine_id = (1 << 16) - 1;

    map.insert("id".into(), id);
    map.insert("time".into(), (id >> 24) as u64);
    map.insert("machine-id".into(), id & mask_machine_id);

    map
}

impl Iterator for Sarmio {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Option::from(self.next_id())
    }
}
