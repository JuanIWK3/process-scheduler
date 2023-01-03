use crate::process::SRTProcess;

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
    processes.push(SRTProcess {
        name: String::from("p1"),
        arrival_time: 2,
        burst_time: &3,
        has_interruption: false,
        stopped: false,
        is_interrupted: false,
        time_spent: 0,
        return_time: 0,
        wait_time: 0,
        completion_time: 0,
    });
    // processes.push(SRTProcess {
    //     name: String::from("p2"),
    //     arrival_time: 4,
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
    //     name: String::from("p3"),
    //     arrival_time: 5,
    //     burst_time: &4,
    //     has_interruption: false,
    //     stopped: false,
    //     return_time: 0,
    //     is_interrupted: false,
    //     time_spent: 0,
    //     wait_time: 0,
    //     completion_time: 0,
    // });

    processes.sort_by_key(|d| (d.burst_time - d.time_spent));

    processes
}
