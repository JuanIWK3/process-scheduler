extern crate queues;

use crate::process::SJFProcess;

pub fn create() -> Vec<SJFProcess<'static>> {
    let mut processes: Vec<SJFProcess> = Vec::new();
    processes.push(SJFProcess {
        name: String::from("p1"),
        duration: &5,
        has_interruption: true,
        time_spent: 0,
    });
    processes.push(SJFProcess {
        name: String::from("p2"),
        duration: &4,
        has_interruption: false,
        time_spent: 0,
    });
    processes.push(SJFProcess {
        name: String::from("p3"),
        duration: &7,
        has_interruption: false,
        time_spent: 0,
    });
    processes.push(SJFProcess {
        name: String::from("p4"),
        duration: &2,
        has_interruption: false,
        time_spent: 0,
    });

    processes.sort_by_key(|d| d.duration);

    processes
}
