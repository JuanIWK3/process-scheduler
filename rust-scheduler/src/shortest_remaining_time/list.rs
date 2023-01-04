use crate::process::SRTProcess;

// ! Don' create a process that lasts shorter than 1 second with interruption
pub fn create() -> Vec<SRTProcess<'static>> {
    let mut processes: Vec<SRTProcess> = Vec::new();
    processes.push(SRTProcess {
        name: String::from("p0"),
        arrival_time: 0,
        burst_time: &5,
        has_interruption: true,
        stopped: false,
        return_time: 0,
        is_interrupted: false,
        time_spent: 0,
        wait_time: 0,
        completion_time: 0,
    });
    // processes.push(SRTProcess {
    //     name: String::from("p1"),
    //     arrival_time: 0,
    //     burst_time: &7,
    //     has_interruption: false,
    //     stopped: false,
    //     return_time: 0,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p0"),
    //     arrival_time: 0,
    //     burst_time: &1,
    //     has_interruption: false,
    //     stopped: false,
    //     return_time: 0,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p1"),
    //     arrival_time: 0,
    //     burst_time: &2,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p2"),
    //     arrival_time: 0,
    //     burst_time: &3,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p3"),
    //     arrival_time: 0,
    //     burst_time: &4,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p4"),
    //     arrival_time: 0,
    //     burst_time: &5,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p5"),
    //     arrival_time: 0,
    //     burst_time: &6,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p6"),
    //     arrival_time: 16,
    //     burst_time: &1,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p7"),
    //     arrival_time: 16,
    //     burst_time: &2,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p8"),
    //     arrival_time: 16,
    //     burst_time: &3,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p9"),
    //     arrival_time: 16,
    //     burst_time: &4,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p10"),
    //     arrival_time: 27,
    //     burst_time: &1,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p11"),
    //     arrival_time: 27,
    //     burst_time: &2,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });
    // processes.push(SRTProcess {
    //     name: String::from("p11"),
    //     arrival_time: 27,
    //     burst_time: &3,
    //     has_interruption: false,
    //     stopped: false,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     return_time: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });

    processes.sort_by_key(|d| (d.burst_time - d.time_spent));

    processes
}