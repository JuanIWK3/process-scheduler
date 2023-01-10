use crate::process::Process;

pub fn create() -> Vec<Process<'static>> {
    let mut processes: Vec<Process> = Vec::new();
    processes.push(Process {
        name: String::from("p0"),
        arrival_time: 0,
        burst_time: &1,
        has_interruption: false,
        stopped: false,
        return_time: 0,
        is_interrupted: false,
        time_spent: 0,
   
        completion_time: 0,
        priority: 5,
    });
    processes.push(Process {
        name: String::from("p2"),
        arrival_time: 0,
        burst_time: &2,
        has_interruption: true,
        stopped: false,
        return_time: 0,
        is_interrupted: false,
        time_spent: 0,
      
        completion_time: 0,
        priority: -4,
    });
    processes.push(Process {
        name: String::from("p3"),
        arrival_time: 0,
        burst_time: &3,
        has_interruption: false,
        stopped: false,
        return_time: 0,
        is_interrupted: false,
        time_spent: 0,
      
        completion_time: 0,
        priority: 2,
    });

    processes.sort_by_key(|d| std::cmp::Reverse(d.burst_time));

    processes
}
