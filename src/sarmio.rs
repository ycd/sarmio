use core::time;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
// use std::{error::Error, thread::current};
use std::fmt;
use std::thread;

#[derive(Clone, Debug)]
pub struct Sarmio {
    time: Arc<Mutex<u64>>,
    machine_id: u64,
    sequence: u64,
}

#[derive(Clone, Debug)]
pub struct ID {
    pub id: u64,
    pub machine_id: u64,
    pub time: u64,
}

impl Iterator for Sarmio {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Option::from(self.next_id())
    }
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
    pub fn next_id(&mut self) -> Option<u64> {
        let mut current_time = self.time.lock().unwrap();
        let mut timestamp = self.get_time();
        if timestamp < *current_time {
            if *current_time - timestamp > 150 {
                return None;
            } else {
                self.sleep((*current_time - timestamp) + 1);
                timestamp = self.get_time();
            }
        } else if timestamp == *current_time {
            self.sequence = (self.sequence + 1) & self.machine_id
        } else {
            self.sequence = 0;
        }
        *current_time = timestamp;
        Option::from((timestamp << 24) | self.sequence << 16 | self.machine_id)
    }

    fn sleep(&self, milis: u64) {
        thread::sleep(time::Duration::from_millis(milis));
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

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\ntime: {}\nmachine_id: {}",
            self.id, self.time, self.machine_id
        )
    }
}

pub fn decompose(id: u64) -> ID {
    let mask_machine_id = (1 << 16) - 1;

    ID {
        id: id,
        time: (id >> 24) as u64,
        machine_id: id & mask_machine_id,
    }
}

impl ID {
    pub fn older(&self, other: &ID) -> bool {
        self.time < other.time
    }

    pub fn same_machine(&self, other: &ID) -> bool {
        self.machine_id == other.machine_id
    }
}
