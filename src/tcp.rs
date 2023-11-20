#[derive(Eq)]
pub(crate) struct Tcp {
    pub pid: usize,
    pub process_name: String,
}

impl PartialOrd for Tcp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.pid.cmp(&other.pid))
    }
}

impl Ord for Tcp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pid.cmp(&other.pid)
    }
}

impl PartialEq for Tcp {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}
