#[derive(Debug, Clone)]
pub struct FCFSProcess {
    pub name: String,
    pub arrival_time: usize,
    pub burst_time: usize,
    pub completion_time: usize,
    pub wait_time: usize,
    pub has_interruption: bool,
    pub is_interrupted: bool,
    pub return_time: usize,
    pub time_spent: usize,
}

#[derive(Debug, Clone)]
pub struct SJFProcess<'a> {
    pub name: String,
    pub arrival_time: usize,
    pub burst_time: &'a usize,
    pub completion_time: usize,
    pub wait_time: usize,
    pub has_interruption: bool,
    pub is_interrupted: bool,
    pub return_time: usize,
    pub time_spent: usize,
}

#[derive(Debug, Clone)]
pub struct SRTProcess<'a> {
    pub name: String,
    pub arrival_time: usize,
    pub burst_time: &'a usize,
    pub has_interruption: bool,
    pub is_interrupted: bool,
    pub stopped: bool,
    pub return_time: usize,
    pub time_spent: usize,
    pub wait_time: usize,
    pub completion_time: usize,
}
