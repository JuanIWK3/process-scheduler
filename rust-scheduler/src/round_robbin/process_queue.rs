use crate::process::Process;

pub fn create_queue() -> Vec<Process<'static>> {
    let mut queue: Vec<Process> = Vec::new();

    let p1 = Process {
        name: String::from("P1"),
        burst_time: &3,
        arrival_time: 0,
        completion_time: 0,
        wait_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
    };

    let p2 = Process {
        name: String::from("P2"),
        burst_time: &4,
        arrival_time: 0,
        completion_time: 0,
        wait_time: 0,
        has_interruption: true,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
    };

    let p3 = Process {
        name: String::from("P3"),
        burst_time: &5,
        arrival_time: 0,
        completion_time: 0,
        wait_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
    };

    queue.push(p1);
    queue.push(p2);
    queue.push(p3);

    queue.sort_by_key(|d| d.arrival_time);

    queue
}
