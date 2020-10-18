use crate::syscalls;
use crate::result::TockResult;

const DRIVER_NUMBER: usize = 0x30003;

mod command {
    pub const RESET: usize = 1;
    pub const SET_MODE: usize = 2;
}

mod opmode {
    pub const SLEEP: usize = 0;
    // pub const STANDBY: usize = 1;
    // pub const FREQUENCYSYNTHESIZER: usize = 2;
    // pub const TRANSMIT: usize = 3;
    pub const RECEIVE: usize = 4;
}

pub struct IsmRadioDriver {  }

impl IsmRadioDriver {
    pub fn reset(&self) -> TockResult<usize> {
        let r = syscalls::command(
            DRIVER_NUMBER,
            command::RESET,
            0,
            0)?;
        Ok(r)
    }

    pub fn sleep(&self) -> TockResult<()> {
        syscalls::command(
            DRIVER_NUMBER,
            command::SET_MODE,
            opmode::SLEEP,
            0)?;
        Ok(())
    }

    pub fn receive(&self) -> TockResult<usize> {
        let r = syscalls::command(
            DRIVER_NUMBER,
            command::SET_MODE,
            opmode::RECEIVE,
            0)?;
        Ok(r)
    }
}
