extern crate queues;
pub mod process_queue;

use crate::first_come_first_served::queues::IsQueue;
use crate::process::FCFSProcess;
use rand::{thread_rng, Rng};
use std::{thread, time};

pub fn init() {
    println!("\n====== FIRST COME FIRST SERVED ======");

    let mut fcfs_queue = process_queue::create_queue();

    let mut time_elapsed = 0;

    while fcfs_queue.size() > 0 {
        let process = fcfs_queue.remove().expect("Error");

        let mut random_time = 0;

        if process.has_interruption {
            let mut rng = thread_rng();
            let random: u32 = rng.gen();
            random_time = (random % (process.duration - 1)) + 1;
        }

        if process.time_spent > 0 {
            println!(
                "Returning process {} at time {}",
                process.name, time_elapsed
            );
        }

        for i in (process.time_spent)..=(process.duration) {
            if process.stopped && process.return_time > time_elapsed {
                println!("cannot return {} at time {}", process.name, time_elapsed);
                thread::sleep(time::Duration::from_secs(1));
                time_elapsed += 1;
                fcfs_queue.add(process).expect("Error adding");

                break;
            }

            if process.stopped && process.return_time > time_elapsed {
                let updated_process = FCFSProcess {
                    stopped: false,
                    ..process.clone()
                };

                fcfs_queue.add(updated_process).expect("Error adding");
            }

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
                // let mut rng = thread_rng();
                // let interruption_time: u32 = rng.gen::<u32>() % process.duration + 1;
                let interruption_time = 5;

                println!(
                    "interruption at {}s for {}s",
                    random_time, interruption_time
                );
                println!(
                    "process can return at {}s",
                    time_elapsed + interruption_time
                );
                println!(
                    "time remaining for process {}: {}s",
                    process.name,
                    process.duration - i
                );

                let updated_process = FCFSProcess {
                    has_interruption: false,
                    time_spent: i + 1,
                    stopped: true,
                    return_time: time_elapsed + interruption_time,
                    ..process.clone()
                };

                fcfs_queue.add(updated_process).expect("Error adding");

                break;
            }

            if i == process.duration {
                println!("Process {} finished at {time_elapsed}s!\n", process.name);
                continue;
            }
        }
    }

    println!("Time elapsed: {} s", time_elapsed)
}
