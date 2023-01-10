use rand::{thread_rng, Rng};
use std::{thread, time::Duration};

use crate::process::{print_table, Process};

pub mod list;

pub fn init() {
    println!("\n====== SHORTEST JOB FIRST ======");

    let mut list = list::create();
    let mut complete: Vec<Process> = Vec::new();
    let mut time_elapsed = 0;
    let process_quantity = list.len();
    let mut random_time = 0;

    while list.len() > 0 {
        list.sort_by_key(|d| d.burst_time);

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

        let mut random_interruption_time = 0;

        // if process.has_interruption {
        //     random_interruption_time = thread_rng().gen_range(1..*process.burst_time);
        // }

        if process.time_spent > 0 {
            process.resume(&mut time_elapsed);
        }

        for time in (process.time_spent)..=(process.burst_time.clone()) {
            if time == 0 {
                process.start(&mut time_elapsed);
                continue;
            }

            if process.has_interruption && time == process.interruption_time {
                process.interrupt(&random_time, &time_elapsed, &time, &mut list);
                break;
            }

            println!("Process {:?} taking {} s", process.name, time);
            thread::sleep(Duration::from_secs(1));

            if time == *process.burst_time {
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
