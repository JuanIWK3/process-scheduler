pub mod first_come_first_served;
pub mod process;
pub mod shortest_job_first;
pub mod shortest_remaining_time;

fn main() {
    first_come_first_served::init();
    // shortest_job_first::init();
    // shortest_remaining_time::init();
}
