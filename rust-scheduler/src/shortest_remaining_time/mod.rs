use colored::Colorize;
use std::{thread, time};

pub mod list;

pub fn init() {
    println!("\n====== SHORTEST JOB FIRST ======");

    let mut list = list::create();
    let process_number = list.len();

    let mut time_elapsed = 0;
    let mut idle = 0;
    let mut turnaround_time = 0;

    while list.len() > 0 {
        let process = list.remove(0);

        if process.arrival_time > time_elapsed {
            println!("Waiting...");
            list.push(process);
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;
            idle += 1;
            continue;
        }

        for i in process.burst_time - process.remaining_time..=(process.burst_time.clone()) {
            if i == 0 {
                println!(
                    "\n{} process {:?} at {time_elapsed} s",
                    format!("[Starting]").green(),
                    process.name
                );

                thread::sleep(time::Duration::from_secs(1));
                time_elapsed += 1;

                continue;
            }

            if &i == process.burst_time {
                println!("Process {:?} took {i} s!", process.name);

                println!(
                    "{} Process {:?} finished at {time_elapsed} s!\n",
                    format!("[Finished]").green(),
                    process.name
                );
                turnaround_time += time_elapsed - process.arrival_time;
                continue;
            }

            println!("Process {:?} taking {i} s", process.name);
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;
        }
    }

    println!("Time elapsed: {} s", time_elapsed);
    println!("Idle time: {} s", idle);
    println!(
        "Average Turnaround time: {} s",
        turnaround_time / process_number
    );
}
