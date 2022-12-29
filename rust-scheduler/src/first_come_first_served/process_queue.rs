extern crate queues;
use queues::*;

use crate::process::FCFSProcess;

pub fn create_queue() -> Queue<FCFSProcess> {
    let mut queue: Queue<FCFSProcess> = queue![];

    let p1 = FCFSProcess {
        name: String::from("p1"),
        duration: 5,
        has_interruption: true,
        time_spent: 0,
    };

    let p2 = FCFSProcess {
        name: String::from("p2"),
        duration: 3,
        has_interruption: false,
        time_spent: 0,
    };

    queue.add(p1).expect("Error adding to queue");
    queue.add(p2).expect("Error adding to queue");

    queue
}
