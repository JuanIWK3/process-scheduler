extern crate queues;
use queues::*;
use rand::{thread_rng, Rng};
use std::{thread, time};

#[derive(Debug, Clone)]
struct Process {
    name: String,
    duration: u32,
    has_interruption: bool,
    time_spent: u32,
}

fn main() {
    let mut q: Queue<Process> = queue![];

    match q.add(Process {
        name: String::from("foo"),
        has_interruption: true,
        duration: 5,
        time_spent: 0,
    }) {
        Err(_) => (),
        Ok(_) => (),
    };
    match q.add(Process {
        name: String::from("bar"),
        has_interruption: false,
        duration: 3,
        time_spent: 0,
    }) {
        Err(_) => (),
        Ok(_) => (),
    };

    let time_elapsed = 0;

    while q.size() > 0 {
        match q.remove() {
            Ok(p) => {
                handle_process(p.clone(), &mut q);
            }
            Err(_) => (),
        };
    }
}

fn handle_process(process: Process, q: &mut Queue<Process>) {
    let mut rng = thread_rng();
    let random: u32 = rng.gen();
    let random_time = random % process.duration;

    for i in (process.time_spent)..=(process.duration) {
        if i == 0 {
            println!("\nInitializing process {:?}", process.name);
            thread::sleep(time::Duration::from_secs(1));
            continue;
        }

        println!("{} {i}", process.name);
        thread::sleep(time::Duration::from_secs(1));

        if i == process.duration {
            println!("Process {} finished successfully!\n", process.name);
        }

        if process.has_interruption && i == random_time {
            println!("interruption at {}", random_time);
            println!(
                "time remaining for process {} {}",
                process.name,
                process.duration - i
            );

            let updated_process = Process {
                has_interruption: false,
                time_spent: i + 1,
                ..process.clone()
            };

            match q.add(updated_process) {
                Ok(_) => (),
                Err(_) => (),
            };

            break;
        }
    }
}
