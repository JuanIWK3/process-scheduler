use crate::process::Process;

pub fn create() -> Vec<Process<'static>> {
    let mut processes: Vec<Process> = Vec::new();
    processes.push(Process {
        name: String::from("p1"),
        arrival_time: 0,
        burst_time: &4,
        completion_time: 0,
        has_interruption: false,
        return_time: 0,
        is_interrupted: false,
        time_spent: 0,
        stopped: false,
        priority: 0,
    });
    processes.push(Process {
        name: String::from("p2"),
        arrival_time: 0,
        burst_time: &3,
        completion_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
        priority: 0,
    });
    processes.push(Process {
        name: String::from("p3"),
        arrival_time: 0,
        burst_time: &3,
        completion_time: 0,
        has_interruption: true,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
        stopped: false,
        priority: 0,
    });

    processes.sort_by_key(|d| (d.burst_time));

    processes
}
