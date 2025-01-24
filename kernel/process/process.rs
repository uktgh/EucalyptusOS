pub enum ProcessState {
    New,
    Ready,
    Running,
    Waiting,
    Terminated,
    Suspended,
}

pub struct Process {
    pub pid: u32,
    pub priority: u32,
    pub state: ProcessState,
    pub context: ProcessContext,
}

#[repr(C)]
pub struct ProcessContext {
    pub esp: u32,
    pub ebp: u32,
    pub ebx: u32,
    pub esi: u32,
    pub edi: u32,
    pub eip: u32,
}

impl Process {
    pub fn new(pid: u32, entry_point: u32, priority: u32) -> Self {
        let context = ProcessContext {
            esp: 0,
            ebp: 0,
            ebx: 0,
            esi: 0,
            edi: 0,
            eip: entry_point,
        };

        Self {
            pid,
            priority,
            state: ProcessState::New,
            context,
        }
    }

    pub fn transition_to_ready(&mut self) {
        self.state = ProcessState::Ready;
    }

    pub fn transition_to_running(&mut self) {
        self.state = ProcessState::Running;
    }

    pub fn transition_to_waiting(&mut self) {
        self.state = ProcessState::Waiting;
    }

    pub fn transition_to_terminated(&mut self) {
        self.state = ProcessState::Terminated;
    }

    pub fn transition_to_suspended(&mut self) {
        self.state = ProcessState::Suspended;
    }
}
