// Module cmdseq.
pub struct CmdSeq {
    times_before_next: usize,
    cmd: String,
}

impl CmdSeq {
    pub fn new(t_b_n: usize, command: String) -> CmdSeq {
        CmdSeq { times_before_next: t_b_n, cmd: command }
    }

    pub fn get_times_before_next(&self) -> usize {
        self.times_before_next
    }

    pub fn get_cmd(&self) -> &str {
        &self.cmd
    }
}
