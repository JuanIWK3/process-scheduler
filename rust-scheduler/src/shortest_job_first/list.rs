use crate::process::SJFProcess;

pub fn create() -> Vec<SJFProcess<'static>> {
    let mut processes: Vec<SJFProcess> = Vec::new();
    processes.push(SJFProcess {
        name: String::from("p1"),
        arrival_time: 0,
        burst_time: &4,
        completion_time: 0,
        wait_time: 0,
        has_interruption: true,
        return_time: 0,
        is_interrupted: false,
        time_spent: 0,
    });
    processes.push(SJFProcess {
        name: String::from("p2"),
        arrival_time: 6,
        burst_time: &2,
        completion_time: 0,
        wait_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
    });
    processes.push(SJFProcess {
        name: String::from("p3"),
        arrival_time: 6,
        burst_time: &3,
        completion_time: 0,
        wait_time: 0,
        has_interruption: false,
        is_interrupted: false,
        return_time: 0,
        time_spent: 0,
    });

    processes.sort_by_key(|d| (d.burst_time));

    processes
}
