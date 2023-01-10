use std::{thread, time::Duration};

use colored::Colorize;
use rand::{thread_rng, Rng};

use crate::process::{print_table, Process};

pub mod list;

pub fn init() {
    println!("\n====== SHORTEST REMAINING TIME ======");

    let mut list = list::create();
    let mut complete: Vec<Process> = Vec::new();
    let mut time_elapsed = 0;
    let mut random_interruption_time = 0;
    let process_quantity = list.len();

    while list.len() > 0 {
        list.sort_by_key(|d| (d.burst_time - d.time_spent));

        let process = match find_process(&list, time_elapsed) {
            Some(index) => list.remove(index),
            None => list.remove(0),
        };

        if process.is_interrupted && process.return_time > time_elapsed {
            process.wait_interruption(&mut time_elapsed, &mut list);
            continue;
        }

        if process.arrival_time > time_elapsed {
            process.wait_arrival(&mut time_elapsed, &mut list);
            continue;
        }

        if process.time_spent > 0 {
            process.resume(&mut time_elapsed);
        }

        if process.has_interruption {
            random_interruption_time = thread_rng().gen_range(1..*process.burst_time);
        }

        for time in (process.time_spent)..=(process.burst_time.clone()) {
            list.sort_by_key(|d| (d.burst_time - d.time_spent));

            match find_faster_process(&list, time_elapsed, &process, time) {
                Some(index) => {
                    list.push(Process {
                        time_spent: time + 1,
                        ..process.clone()
                    });

                    println!(
                        "{} Process {} interrupted by process {}: {} s",
                        format!("[Warn]").yellow(),
                        process.name,
                        list[index].name,
                        list[index].burst_time - list[0].time_spent
                    );

                    println!(
                        "{} Remaining time for {} is {} s",
                        format!("[Warn]").yellow(),
                        process.name,
                        process.burst_time - time
                    );
                    break;
                }
                None => (),
            };

            if time == 0 {
                process.start(&mut time_elapsed);
                continue;
            }

            if process.has_interruption && time == random_interruption_time {
                process.interrupt(&random_interruption_time, &time_elapsed, &time, &mut list);
                break;
            }

            println!("Process {:?} taking {} s", process.name, time);
            thread::sleep(Duration::from_secs(1));

            if &time == process.burst_time {
                process.end(&mut time_elapsed, &mut complete);
                continue;
            }

            time_elapsed += 1;
        }
    }

    print_table(&time_elapsed, &mut complete, &process_quantity);
}

fn find_process(list: &Vec<Process>, time_elapsed: usize) -> Option<usize> {
    for i in 0..list.len() {
        if list[i].arrival_time <= time_elapsed && list[i].return_time <= time_elapsed {
            println!("Found {}", list[i].name);
            return Some(i);
        }
    }

    None
}

fn find_faster_process(
    list: &Vec<Process>,
    time_elapsed: usize,
    current_process: &Process,
    time: usize,
) -> Option<usize> {
    for i in 0..list.len() {
        if list[i].arrival_time <= time_elapsed
            && list[i].burst_time - list[i].time_spent < current_process.burst_time - time
            && list[i].return_time <= time_elapsed
        {
            return Some(i);
        }
    }

    None
}
