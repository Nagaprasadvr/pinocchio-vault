use pinocchio::program_error::ProgramError;

pub mod deposit;
pub mod initialize_mystate;
pub mod update_mystate;
pub mod withdraw;

pub use initialize_mystate::*;
pub use update_mystate::*;

#[repr(u8)]
pub enum MyProgramInstrution {
    InitializeState,
    UpdateState,
}

impl TryFrom<&u8> for MyProgramInstrution {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MyProgramInstrution::InitializeState),
            1 => Ok(MyProgramInstrution::UpdateState),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
