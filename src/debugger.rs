/// Debugger message type
#[derive(Debug)]
pub enum MessageType {
    /// Line executed (instruction, consumed cycles)
    LineExecuted(u8, u32),
    /// Waiting for input
    Waiting,
    /// Overflow
    Overflow,
    /// Breakpoint hit
    BreakpointHit,
}

/// Debugger
#[allow(missing_debug_implementations)]
pub struct Debugger<E> {
    /// Debugger callback [``]
    pub messenger: E,
    #[allow(dead_code)]
    pc: u16,
    /// Breakpoints
    #[allow(dead_code)]
    wait_next_instruction: bool,
    #[allow(dead_code)]
    halt: bool,
    /// Breakpoints
    #[allow(dead_code)]
    pub breakpoints: Vec<u16>,
}

impl<E> Debugger<E> {
    /// Create a new debugger
    /// ## Arguments
    /// * `messenger` - Debugger callback [`FnOnce(MessageType)`]
    pub fn new(messenger: E) -> Self {
        Debugger {
            messenger,
            pc: 0,
            wait_next_instruction: false,
            halt: false,
            breakpoints: Vec::new(),
        }
    }
}
