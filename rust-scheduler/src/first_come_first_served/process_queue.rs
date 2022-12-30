extern crate queues;
use queues::*;

use crate::process::FCFSProcess;

pub fn create_queue() -> Queue<FCFSProcess> {
    let mut queue: Queue<FCFSProcess> = queue![];

    let p1 = FCFSProcess {
        name: String::from("P1"),
        duration: 6,
        has_interruption: true,
        time_spent: 0,
        return_time: 0,
        stopped: false,
    };

    let p2 = FCFSProcess {
        name: String::from("P2"),
        duration: 2,
        has_interruption: false,
        time_spent: 0,
        return_time: 0,
        stopped: false,
    };

    queue.add(p1).expect("Error adding to queue");
    queue.add(p2).expect("Error adding to queue");

    queue
}
