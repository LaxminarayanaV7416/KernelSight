#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ProcFSPID {
    pub pid: u64,
}

pub enum PIDActions {
    Add(ProcFSPID),
    Remove(ProcFSPID),
}
