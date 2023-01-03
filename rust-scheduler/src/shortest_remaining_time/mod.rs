use std::{
    thread,
    time::{self, Duration},
};

use colored::Colorize;

use crate::process::SRTProcess;

pub mod list;

pub fn init() {
    println!("\n====== SHORTEST REMAINING TIME ======");

    let mut list = list::create();
    let mut time_elapsed = 0;

    while list.len() > 0 {
        list.sort_by_key(|d| d.burst_time);

        let mut process = match find_process(&list, time_elapsed) {
            Some(index) => list.remove(index),
            None => list.remove(0),
        };

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

        for i in (process.time_spent)..=(process.burst_time.clone()) {
            list.sort_by_key(|d| d.burst_time);

            match find_faster_process(&list, time_elapsed, &process) {
                Some(_) => {
                    list.push(SRTProcess {
                        time_spent: i,
                        ..process
                    });
                    break;
                }
                None => (),
            };

            if i == 0 {
                start_process(&process, &mut time_elapsed);
                continue;
            }

            if &i == process.burst_time {
                end_process(&process, &mut time_elapsed, &i);
                continue;
            }

            println!("Process {:?} taking {i} s", process.name);
            thread::sleep(Duration::from_secs(1));
            time_elapsed += 1;
        }
    }

    println!("\nTime elapsed: {} s\n", time_elapsed);
}

fn start_process(process: &SRTProcess, time_elapsed: &mut usize) {
    println!(
        "\n{} process {:?} at {time_elapsed} s",
        format!("[Starting]").green(),
        process.name
    );

    thread::sleep(time::Duration::from_secs(1));
    *time_elapsed += 1;
}

fn end_process(process: &SRTProcess, time_elapsed: &mut usize, current_time: &usize) {
    println!("Process {:?} took {current_time} s!", process.name);

    println!(
        "{} Process {:?} finished at {time_elapsed} s!",
        format!("[Finished]").green(),
        process.name
    );
}

fn find_process(list: &Vec<SRTProcess>, time_elapsed: usize) -> Option<usize> {
    for i in 0..list.len() {
        if list[i].arrival_time <= time_elapsed {
            return Some(i);
        }
    }

    None
}

fn find_faster_process(
    list: &Vec<SRTProcess>,
    time_elapsed: usize,
    current_process: &SRTProcess,
) -> Option<usize> {
    for i in 0..list.len() {
        if list[i].arrival_time < time_elapsed {
            if list[i].burst_time < current_process.burst_time {
                return Some(i);
            }
        }
    }

    None
}
