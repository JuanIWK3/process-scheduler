pub mod process_queue;
use colored::Colorize;
use rand::{thread_rng, Rng};

use std::{thread, time};

use crate::process::FCFSProcess;

pub fn init() {
    println!("\n====== FIRST COME FIRST SERVED ======");

    let mut fcfs_queue = process_queue::create_queue();
    let mut complete: Vec<FCFSProcess> = Vec::new();
    let mut time_elapsed = 0;
    let process_quantity = fcfs_queue.len();

    while fcfs_queue.len() > 0 {
        let process = fcfs_queue.remove(0);

        if process.is_interrupted && process.return_time > time_elapsed {
            println!("Waiting for process to return...");
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;
            fcfs_queue.push(FCFSProcess {
                wait_time: process.wait_time + 1,
                ..process
            });
            continue;
        }

        // See if the process is ready
        if process.arrival_time > time_elapsed {
            println!("Waiting...");
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;
            fcfs_queue.push(FCFSProcess {
                wait_time: process.wait_time + 1,
                ..process
            });
            continue;
        }

        let mut random_time = 0;

        if process.has_interruption {
            random_time = thread_rng().gen_range(1..process.burst_time);
        }

        if process.time_spent > 0 {
            println!(
                "Returning process {} at time {}",
                process.name, time_elapsed
            );
        }

        // Handle the process execution
        for i in (process.time_spent)..=(process.burst_time) {
            println!("at time {}", time_elapsed);
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

            // Wait for the process return
            if process.is_interrupted && process.return_time > time_elapsed {
                println!(
                    "{} Process {:?} will return in {} s...",
                    format!("[Warn]").yellow(),
                    process.name,
                    process.return_time - time_elapsed
                );
                thread::sleep(time::Duration::from_secs(1));
                time_elapsed += 1;
                fcfs_queue.push(process);

                break;
            }

            // Jump to the next line when the process is ready to return
            if process.is_interrupted && (time_elapsed == process.return_time) {
                println!("");
            }

            if process.has_interruption && i == random_time {
                // let mut rng = thread_rng();
                // let interruption_time: u32 = rng.gen::<u32>() % process.duration + 1;
                let interruption_time = 4;

                println!("Process {:?} taking {i} s", process.name);
                println!(
                    "\n{} Interruption at {} s for {} s",
                    format!("[Warn]").yellow(),
                    random_time,
                    interruption_time
                );
                println!(
                    "{} Process can return at {} s",
                    format!("[Warn]").yellow(),
                    time_elapsed + interruption_time
                );
                println!(
                    "{} Time remaining for process {}: {} s",
                    format!("[Warn]").yellow(),
                    process.name,
                    process.burst_time - i
                );

                let updated_process = FCFSProcess {
                    has_interruption: false,
                    time_spent: i + 1,
                    is_interrupted: true,
                    return_time: time_elapsed + interruption_time,
                    ..process.clone()
                };

                fcfs_queue.push(updated_process);

                break;
            }

            if i == process.burst_time {
                time_elapsed += 1;
                thread::sleep(time::Duration::from_secs(1));

                println!("Process {:?} took {i} s!", process.name);

                println!(
                    "{} Process {:?} finished at {time_elapsed} s!\n",
                    format!("[Finished]").green(),
                    process.name
                );
                complete.push(FCFSProcess {
                    completion_time: time_elapsed,
                    ..process.clone()
                });
                continue;
            }

            println!("Process {:?} taking {i} s", process.name);
            time_elapsed += 1;
            thread::sleep(time::Duration::from_secs(1));
        }
    }

    // Print complete table

    println!("Time elapsed: {} s\n", time_elapsed);

    println!("Process\tArrival\tBurst\tCompletion\tTurnaround\tWaiting");

    let mut total_turnaround_time = 0;
    let mut total_waiting_time = 0;

    for process in complete {
        println!(
            "{}\t{}\t{}\t{}\t\t{}\t\t{}",
            process.name,
            process.arrival_time,
            process.burst_time,
            process.completion_time,
            process.completion_time - process.arrival_time,
            process.wait_time
        );

        total_turnaround_time += process.completion_time - process.arrival_time;
        total_waiting_time += process.wait_time;
    }

    println!(
        "\nAverage Turnaround Time: {:?}",
        total_turnaround_time / process_quantity
    );
    println!("Average Waiting Time: {:?}", total_waiting_time)
}
