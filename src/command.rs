/// A list of command types that we know how to process
#[derive(Copy, Clone, Debug)]
pub enum CommandType {
    MouseMoved,
    MouseDown,
    MouseUp,
    KeyDown,
    MouseScroll,
}

/// A command that can be queued with the data that came with it.
#[derive(Copy, Clone)]
pub struct Command {
    /// The type of the command
    pub command_type: CommandType,
    /// A data point that may be useful - e.g. X for mouse position.
    pub data1: u32,
    /// A data point that may be useful - e.g. Y for mouse position.
    pub data2: u32,
}
