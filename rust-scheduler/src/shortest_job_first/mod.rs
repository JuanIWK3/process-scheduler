use colored::Colorize;
use std::{thread, time};

use crate::process::SJFProcess;

pub mod list;

pub fn init() {
    println!("\n====== SHORTEST JOB FIRST ======");

    let mut list = list::create();
    let mut complete: Vec<SJFProcess> = Vec::new();
    let process_number = list.len();

    let mut time_elapsed = 0;

    while list.len() > 0 {
        let process = list.remove(0);

        if process.arrival_time > time_elapsed {
            println!("Waiting...");
            list.push(SJFProcess {
                wait_time: process.wait_time + 1,
                ..process
            });
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;
            continue;
        }

        for i in 0..=(process.burst_time.clone()) {
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
                    "{} Process {:?} finished at {time_elapsed} s!",
                    format!("[Finished]").green(),
                    process.name
                );
                complete.push(SJFProcess {
                    completion_time: time_elapsed,
                    ..process.clone()
                });
                continue;
            }

            println!("Process {:?} taking {i} s", process.name);
            thread::sleep(time::Duration::from_secs(1));
            time_elapsed += 1;
        }
    }

    println!("\nTime elapsed: {} s\n", time_elapsed);

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
            process.completion_time - process.burst_time
        );

        total_turnaround_time += process.completion_time - process.arrival_time;
        total_waiting_time += process.completion_time - process.burst_time;
    }

    println!(
        "\nAverage Turnaround Time: {:?}",
        total_turnaround_time / process_number
    );
    println!("Average Waiting Time: {:?}", total_waiting_time)
}
