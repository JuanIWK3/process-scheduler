pub mod list;
use rand::{thread_rng, Rng};

use std::{thread, time};

use crate::process::{print_table, Process};

pub fn init() {
    println!("\n====== ROUND ROBBIN ======");

    let mut list = list::create_queue();
    let mut complete: Vec<Process> = Vec::new();
    let mut time_elapsed = 0;
    let mut random_interruption_time = 0;
    let process_quantity = list.len();
    let quantum = 2;

    while list.len() > 0 {
        let process = list.remove(0);

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

        let mut quantum_count: usize = 0;

        // Handle the process execution
        for time in (process.time_spent)..=(*process.burst_time) {
            if time == 0 {
                process.start(&mut time_elapsed);
                continue;
            }

            if process.has_interruption && time == random_interruption_time {
                process.interrupt(&random_interruption_time, &time_elapsed, &time, &mut list);
                break;
            }

            if &time == process.burst_time {
                process.end(&mut time_elapsed, &mut complete);
                continue;
            }

            println!("Process {:?} taking {time} s", process.name);
            time_elapsed += 1;
            thread::sleep(time::Duration::from_secs(1));

            quantum_count += 1;

            if quantum_count == quantum {
                let updated_process = Process {
                    time_spent: time + 1,
                    ..process.clone()
                };

                list.push(updated_process);
                break;
            }
        }
    }

    print_table(&time_elapsed, &mut complete, &process_quantity);
}
