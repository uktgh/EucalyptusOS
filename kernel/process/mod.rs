pub mod scheduler;
pub mod process;

pub struct ProcessManager {
    processes: Vec<Process>,
    scheduler: Scheduler,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn create_process(&mut self, pid: u32, entry_point: u32, priority: u32) -> &mut Process {
        let process = Process::new(pid, entry_point, priority);
        self.processes.push(process);
        self.scheduler.add_process(process);
        self.processes.last_mut().unwrap()
    }

    pub fn kill_process(&mut self, pid: u32) {
        if let Some(index) = self.processes.iter().position(|p| p.pid == pid) {
            self.processes[index].transition_to_terminated();
            self.scheduler.remove_process(pid);
            self.processes.remove(index);
        }
    }

    pub fn get_process(&self, pid: u32) -> Option<&Process> {
        self.processes.iter().find(|p| p.pid == pid)
    }

    pub fn get_process_mut(&mut self, pid: u32) -> Option<&mut Process> {
        self.processes.iter_mut().find(|p| p.pid == pid)
    }
}
