use std::{thread, time::Duration};

use colored::Colorize;
use rand::{thread_rng, Rng};

use crate::process::Process;

pub mod list;

pub fn init() {
    println!("\n====== DULLING ======");

    let mut list = list::create();
    let mut time_elapsed = 0;
    let mut random_interruption_time = 0;

    while list.len() > 0 {
        list.sort_by_key(|d| std::cmp::Reverse(d.burst_time));

        let process = match find_process(&list, time_elapsed) {
            Some(index) => list.remove(index),
            None => list.remove(0),
        };

        if process.is_interrupted && process.return_time > time_elapsed {
            println!("Waiting for process to return...");
            thread::sleep(Duration::from_secs(1));
            time_elapsed += 1;
            list.push(Process {
                wait_time: process.wait_time + 1,
                ..process
            });
            continue;
        }

        if process.arrival_time > time_elapsed {
            println!("Process not ready");

            if false {
            } else {
                thread::sleep(Duration::from_secs(1));
                time_elapsed += 1;
            }

            list.push(process);

            continue;
        }

        if process.time_spent > 0 {
            println!(
                "\n{} process {} at {} s",
                format!("[Resuming]").green(),
                process.name,
                time_elapsed
            );
        }

        if process.has_interruption {
            random_interruption_time = thread_rng().gen_range(1..*process.burst_time);
        }

        for time in (process.time_spent)..=(process.burst_time.clone()) {
            list.sort_by_key(|d| std::cmp::Reverse(d.burst_time));

            if time == 0 {
                start_process(&process, &mut time_elapsed);
                continue;
            }

            if &time == process.burst_time {
                end_process(&process, &mut time_elapsed, &time);
                continue;
            }

            if process.has_interruption && time == random_interruption_time {
                // let mut rng = thread_rng();
                // let interruption_time: u32 = rng.gen::<u32>() % process.duration + 1;
                time_elapsed += 1;

                let interruption_time = 2;
                println!("Process {:?} taking {} s", process.name, time);
                println!(
                    "\n{} Interruption at {} s for {} s",
                    format!("[Warn]").yellow(),
                    random_interruption_time,
                    interruption_time
                );
                println!(
                    "{} Process can return at {} s",
                    format!("[Warn]").yellow(),
                    time_elapsed + interruption_time - 1
                );
                println!(
                    "{} Time remaining for process {}: {} s",
                    format!("[Warn]").yellow(),
                    process.name,
                    process.burst_time - time
                );

                let updated_process = Process {
                    has_interruption: false,
                    time_spent: time,
                    is_interrupted: true,
                    return_time: time_elapsed + interruption_time,
                    ..process.clone()
                };

                list.push(updated_process);

                break;
            }

            println!("Process {:?} taking {} s", process.name, time);
            thread::sleep(Duration::from_secs(1));
            time_elapsed += 1;
        }
    }

    println!("\nTime elapsed: {} s", time_elapsed);
}

fn start_process(process: &Process, time_elapsed: &mut usize) {
    println!(
        "\n{} process {:?} at {time_elapsed} s",
        format!("[Starting]").green(),
        process.name
    );

    *time_elapsed += 1;
}

fn end_process(process: &Process, time_elapsed: &mut usize, current_time: &usize) {
    println!("Process {:?} took {current_time} s!", process.name);

    println!(
        "{} Process {:?} finished at {time_elapsed} s!",
        format!("[Finished]").green(),
        process.name
    );

    *time_elapsed += 1;
}

fn find_process(list: &Vec<Process>, time_elapsed: usize) -> Option<usize> {
    for i in 0..list.len() {
        if list[i].arrival_time <= time_elapsed && !list[i].is_interrupted {
            return Some(i);
        }
    }

    None
}
