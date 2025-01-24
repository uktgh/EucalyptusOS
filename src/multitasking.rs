struct Process {
    pid: u32,
    stack_pointer: u32,
    state: ProcessState,
}

enum ProcessState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

impl Process {
    fn new(pid: u32, entry_point: u32) -> Self {
        Self {
            pid,
            stack_pointer: entry_point,
            state: ProcessState::Ready,
        }
    }
}

struct Scheduler {
    processes: Vec<Process>,
    current_process: usize,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_process: 0,
        }
    }

    fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    fn switch(&mut self) {
        if !self.processes.is_empty() {
            self.processes[self.current_process].state = ProcessState::Ready;
            self.current_process = (self.current_process + 1) % self.processes.len();
            self.processes[self.current_process].state = ProcessState::Running;
        }
    }
}
