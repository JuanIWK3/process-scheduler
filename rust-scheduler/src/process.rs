use core::time;
use std::{thread, time::Duration};

use colored::Colorize;
// use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Process<'a> {
    pub name: String,
    pub arrival_time: usize,
    pub burst_time: &'a usize,
    pub has_interruption: bool,
    pub is_interrupted: bool,
    pub stopped: bool,
    pub return_time: usize,
    pub time_spent: usize,
    pub completion_time: usize,
    pub priority: i32,
    pub interruption_time: usize,
    pub interruption_duration: usize,
}

impl Process<'static> {
    pub fn start(&self, time_elapsed: &mut usize) {
        println!(
            "\n{} process {:?} at {time_elapsed} s",
            format!("[Starting]").green(),
            self.name
        );

        *time_elapsed += 1;
        thread::sleep(Duration::from_secs(1));
    }

    pub fn end(&self, time_elapsed: &mut usize, complete: &mut Vec<Process>) {
        println!(
            "{} Process {:?} finished at {time_elapsed} s!",
            format!("[Finished]").green(),
            self.name
        );

        complete.push(Process {
            completion_time: *time_elapsed,
            ..self.clone()
        });
    }

    pub fn resume(&self, time_elapsed: &mut usize) {
        println!(
            "\n{} process {} at {} s",
            format!("[Resuming]").green(),
            self.name,
            time_elapsed
        );

        *time_elapsed += 1;
    }

    pub fn wait_arrival(&self, time_elapsed: &mut usize, list: &mut Vec<Process<'static>>) {
        println!("Waiting for process to arrive...");
        thread::sleep(Duration::from_secs(1));
        *time_elapsed += 1;
        list.push(self.clone());
    }

    pub fn wait_interruption(&self, time_elapsed: &mut usize, list: &mut Vec<Process<'static>>) {
        println!("Waiting for process to return...");
        thread::sleep(time::Duration::from_secs(1));
        *time_elapsed += 1;
        list.push(self.clone());
    }

    pub fn interrupt(
        &self,
        _random_interruption_time: &usize,
        time_elapsed: &usize,
        time: &usize,
        list: &mut Vec<Process<'static>>,
    ) {
        // let mut rng = thread_rng();
        // let interruption_duration = rng.gen::<usize>() % self.burst_time + 1;
        let interruption_duration = self.interruption_duration;

        println!("Process {:?} taking {} s", self.name, time);
        println!(
            "\n{} Interruption at {} s for {} s",
            format!("[Warn]").yellow(),
            self.interruption_time,
            self.interruption_duration
        );

        println!(
            "{} Process can return at {} s",
            format!("[Warn]").yellow(),
            time_elapsed + interruption_duration
        );

        println!(
            "{} Time remaining for process {}: {} s",
            format!("[Warn]").yellow(),
            self.name,
            self.burst_time - time
        );

        let updated_process = Process {
            has_interruption: false,
            time_spent: time + 1,
            is_interrupted: true,
            return_time: time_elapsed + interruption_duration,
            ..self.clone()
        };

        list.push(updated_process);
    }
}

pub fn print_table(time_elapsed: &usize, complete: &mut Vec<Process>, process_quantity: &usize) {
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
        total_turnaround_time / process_quantity
    );
    println!(
        "Average Waiting Time: {:?}",
        total_waiting_time / process_quantity
    )
}
