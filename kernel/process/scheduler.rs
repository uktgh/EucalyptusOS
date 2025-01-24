use super::process::{Process, ProcessState};

pub struct Scheduler {
    processes: Vec<Process>,
    current_process: Option<u32>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_process: None,
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    pub fn remove_process(&mut self, pid: u32) {
        if let Some(index) = self.processes.iter().position(|p| p.pid == pid) {
            self.processes.remove(index);
        }
    }

    pub fn schedule(&mut self) {
        if let Some(current_pid) = self.current_process {
            if let Some(current_index) = self.processes.iter().position(|p| p.pid == current_pid) {
                self.processes[current_index].transition_to_ready();
            }
        }

        if let Some(next_process) = self.processes.iter_mut().find(|p| p.state == ProcessState::Ready) {
            next_process.transition_to_running();
            self.current_process = Some(next_process.pid);
            self.switch_to(next_process);
        }
    }

    pub fn switch_to(&self, process: &Process) {
        unsafe {
            asm!(
                "mov esp, {0}",
                "mov ebp, {1}",
                "mov ebx, {2}",
                "mov esi, {3}",
                "mov edi, {4}",
                "jmp {5}",
                in(reg) process.context.esp,
                in(reg) process.context.ebp,
                in(reg) process.context.ebx,
                in(reg) process.context.esi,
                in(reg) process.context.edi,
                in(reg) process.context.eip,
            );
        }
    }
}
