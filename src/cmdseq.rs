// Module cmdseq
use string_utils::{ collect_between_white, count_white_space };

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

pub fn get_command_list(command: &str) -> (Vec<CmdSeq>, usize) {
    let mut command_list: Vec<CmdSeq> = Vec::new();
    // The number of operations we would execute before we need to start from beginning.
    let mut num_operations: usize = 0;
    let mut num = 0;
    while num <= count_white_space(command) { // Using 'while' loop as 'step_by()' has issues atm.
        let t_b_n: usize = collect_between_white(command, num, num + 1).parse().expect("Usage: cmdseq [-d <count dir>] <count1> <cmd1> [... <countn> <cmdn>]");
        num_operations += t_b_n;
        command_list.push(CmdSeq::new(
            t_b_n,
            collect_between_white(command, num + 1, num + 2).replace("\"", ""),
        ));
        num = num + 2;
    }
    (command_list, num_operations)
}
