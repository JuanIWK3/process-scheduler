use crate::process::Process;

pub fn create_queue() -> Vec<Process<'static>> {
    let mut list: Vec<Process> = Vec::new();

    list.push(Process {
        name: String::from("P1"),
        burst_time: &5,
        arrival_time: 0,
        completion_time: 0,
        has_interruption: true,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
        priority: 0,
    });

    list.push(Process {
        name: String::from("P2"),
        burst_time: &1,
        arrival_time: 0,
        completion_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
        priority: 0,
    });

    list.push(Process {
        name: String::from("P3"),
        burst_time: &1,
        arrival_time: 0,
        completion_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
        priority: 0,
    });

    list.sort_by_key(|d| d.arrival_time);

    list
}
