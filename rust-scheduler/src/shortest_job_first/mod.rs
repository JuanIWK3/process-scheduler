extern crate queues;

use crate::process::SJFProcess;
use rand::{thread_rng, Rng};
use std::{thread, time};

pub mod list;

pub fn init() {
    println!("\n====== SHORTEST JOB FIRST ======");

    let mut processes = list::create();

    let mut time_elapsed = 0;

    while processes.len() > 0 {
        let process = processes.remove(0);
        processes.sort_by_key(|d| d.duration);

        let mut random_time = 0;

        if process.has_interruption {
            let mut rng = thread_rng();
            let random: u32 = rng.gen();
            random_time = (random % (process.duration - 1)) + 1;
        }

        for i in (process.time_spent)..=(process.duration.clone()) {
            if i == 0 {
                println!(
                    "\nInitializing process {:?} at {time_elapsed}s",
                    process.name
                );

                continue;
            }

            println!("{} {i}", process.name);
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;

            if process.has_interruption && i == random_time {
                println!("interruption at: {}s", random_time);
                println!(
                    "time remaining for process {}: {}s",
                    process.name,
                    process.duration - i
                );

                let updated_process = SJFProcess {
                    has_interruption: false,
                    time_spent: i + 1,
                    ..process.clone()
                };

                processes.push(updated_process);

                break;
            }

            if &i == process.duration {
                println!("Process {} finished at {time_elapsed}s!\n", process.name);
                continue;
            }
        }
    }

    println!("{}", time_elapsed);
}
