use crate::command::Command;
use crate::command::CommandType;

/// A queue structure so we can queue commands and process them later.
pub struct CommandQueue {
    /// The current number of queued commands
    pub pending_count: usize,
    /// The current command we are processing
    pub current_index: usize,
    /// The list of commands that are queued.
    pub commands: [Command; 100],
}

impl CommandQueue {
    /// Create a new empty command queue.
    pub const fn new() -> CommandQueue {
        CommandQueue {
            pending_count: 0,
            current_index: 0,
            commands: [Command {
                command_type: CommandType::MouseMoved,
                data1: 0,
                data2: 0,
            }; 100],
        }
    }

    /// Pop the next command from the start of the queue.
    pub fn next(&mut self) -> Option<Command> {
        if self.pending_count == 0 {
            None
        } else {
            self.pending_count -= 1;
            let return_index = self.current_index;
            self.current_index = (self.current_index + 1) % self.commands.len();
            Some(self.commands[return_index])
        }
    }

    /// Push a new command on the queue.
    pub fn queue_command(&mut self, command: Command) {
        if self.pending_count < self.commands.len() {
            let next_index = (self.current_index + self.pending_count) % self.commands.len();
            self.commands[next_index] = command;
            self.pending_count += 1;
        }
    }
}
