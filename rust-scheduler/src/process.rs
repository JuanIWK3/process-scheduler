#[derive(Debug, Clone)]
pub struct FCFSProcess {
    pub name: String,
    pub duration: u32,
    pub has_interruption: bool,
    pub time_spent: u32,
}

#[derive(Debug, Clone)]
pub struct SJFProcess<'a> {
    pub name: String,
    pub duration: &'a u32,
    pub has_interruption: bool,
    pub time_spent: u32,
}