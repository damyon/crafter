use crate::operating_system::OperatingSystem;

/// A list of actions that we want to handle
#[derive(Copy, Clone, Debug)]
pub enum Action {
    Forward,
    OpenScene,
    SaveScene,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveBackward,
    MoveForward,
    ToggleVoxel,
    MoveSelectionLeft,
    MoveSelectionRight,
    MoveSelectionUp,
    MoveSelectionDown,
    MoveSelectionForward,
    MoveSelectionBackward,
    ToggleSelectionShape,
    ToggleFluid,
    ToggleShowGrid,
    ToggleNoise,
}

/// A list of virtual key codes that we want to handle
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VirtualKeyCode {
    OpenScene = 4001,
    SaveScene = 4002,
    ToggleShowGrid = 4003,
    ToggleNoise = 4004,
    ToggleFluid = 4005,
    ToggleSelectionShape = 4006,
}

impl VirtualKeyCode {
    fn from_u32(value: u32) -> Option<VirtualKeyCode> {
        match value {
            4001 => Some(VirtualKeyCode::OpenScene),
            4002 => Some(VirtualKeyCode::SaveScene),
            4003 => Some(VirtualKeyCode::ToggleShowGrid),
            4004 => Some(VirtualKeyCode::ToggleNoise),
            4005 => Some(VirtualKeyCode::ToggleFluid),
            4006 => Some(VirtualKeyCode::ToggleSelectionShape),
            _ => None, // Returns None for unknown values
        }
    }
}

pub struct KeyBindings {
    pub os: OperatingSystem
}

impl KeyBindings {
    /// Create new empty KeyBindings.
    pub const fn new() -> KeyBindings {
        #[cfg(target_os = "linux")]
        {
            KeyBindings {
                os: OperatingSystem::Linux
            }
        }
        #[cfg(target_os = "macos")]
        {
            KeyBindings {
                os: OperatingSystem::Mac
            }
        }
    }

    pub fn virtual_key(&self, action: Option<Action>) -> Option<VirtualKeyCode> {
        match action {
            Some(Action::ToggleShowGrid) => Some(VirtualKeyCode::ToggleShowGrid),
            Some(Action::ToggleNoise) => Some(VirtualKeyCode::ToggleNoise),
            Some(Action::ToggleFluid) => Some(VirtualKeyCode::ToggleFluid),
            Some(Action::ToggleSelectionShape) => Some(VirtualKeyCode::ToggleSelectionShape),
            _ => None
        }
    }

    pub fn action(&self, code: u32) -> Option<Action> {
        let virtual_code_opt = VirtualKeyCode::from_u32(code);

        match virtual_code_opt {
            Some(VirtualKeyCode::OpenScene) => Some(Action::OpenScene),
            Some(VirtualKeyCode::SaveScene) => Some(Action::SaveScene),
            Some(VirtualKeyCode::ToggleShowGrid) => Some(Action::ToggleShowGrid),
            Some(VirtualKeyCode::ToggleNoise) => Some(Action::ToggleNoise),
            Some(VirtualKeyCode::ToggleFluid) => Some(Action::ToggleFluid),
            Some(VirtualKeyCode::ToggleSelectionShape) => Some(Action::ToggleSelectionShape),
            None => {
                if self.os == OperatingSystem::Linux {
                    return match code {
                        17 => Some(Action::Forward),
                        16 => Some(Action::MoveUp),
                        18 => Some(Action::MoveDown),
                        30 => Some(Action::MoveLeft),
                        32 => Some(Action::MoveRight),
                        31 => Some(Action::MoveBackward),
                        17 => Some(Action::MoveForward),
                        57 => Some(Action::ToggleVoxel),
                        75 => Some(Action::MoveSelectionLeft),
                        77 => Some(Action::MoveSelectionRight),
                        71 => Some(Action::MoveSelectionUp),
                        73 => Some(Action::MoveSelectionDown),
                        72 => Some(Action::MoveSelectionForward),
                        76 => Some(Action::MoveSelectionBackward),
                        20 => Some(Action::ToggleSelectionShape),
                        33 => Some(Action::ToggleFluid),
                        34 => Some(Action::ToggleShowGrid),
                        49 => Some(Action::ToggleNoise),
                        _ => None
                    }
                }
                if self.os == OperatingSystem::Mac {
                    return match code {
                        126 => Some(Action::Forward),
                        12 => Some(Action::MoveUp),
                        14 => Some(Action::MoveDown),
                        0 => Some(Action::MoveLeft),
                        2 => Some(Action::MoveRight),
                        1 => Some(Action::MoveBackward),
                        13 => Some(Action::MoveForward),
                        49 => Some(Action::ToggleVoxel),
                        38 => Some(Action::MoveSelectionLeft),
                        37 => Some(Action::MoveSelectionRight),
                        32 => Some(Action::MoveSelectionUp),
                        31 => Some(Action::MoveSelectionDown),
                        34 => Some(Action::MoveSelectionForward),
                        40 => Some(Action::MoveSelectionBackward),
                        17 => Some(Action::ToggleSelectionShape),
                        3 => Some(Action::ToggleFluid),
                        5 => Some(Action::ToggleShowGrid),
                        45 => Some(Action::ToggleNoise),
                        _ => None
                    }
                }
                return None
            }
        }
    }
}
