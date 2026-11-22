pub type ProcFSPID = u64;

pub enum PIDActions {
    Add(ProcFSPID),
    Remove(ProcFSPID),
}
