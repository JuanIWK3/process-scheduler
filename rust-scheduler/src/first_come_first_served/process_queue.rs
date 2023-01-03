use crate::process::FCFSProcess;

pub fn create_queue() -> Vec<FCFSProcess> {
    let mut queue: Vec<FCFSProcess> = Vec::new();

    let p1 = FCFSProcess {
        name: String::from("P1"),
        burst_time: 5,
        arrival_time: 0,
        completion_time: 0,
        wait_time: 0,
        has_interruption: true,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
    };

    let p2 = FCFSProcess {
        name: String::from("P2"),
        burst_time: 1,
        arrival_time: 0,
        completion_time: 0,
        wait_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
    };

    let p3 = FCFSProcess {
        name: String::from("P3"),
        burst_time: 1,
        arrival_time: 0,
        completion_time: 0,
        wait_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
    };

    queue.push(p1);
    queue.push(p2);
    queue.push(p3);

    queue.sort_by_key(|d| d.arrival_time);

    queue
}
