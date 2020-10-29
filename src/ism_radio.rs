use crate::syscalls;
use crate::result::TockResult;

const DRIVER_NUMBER: usize = 0x30003;

#[allow(unused)]
mod command {
    pub const AVAILABLE: usize = 0;
    pub const RESET: usize = 1;
    pub const STATUS: usize = 2;
    pub const READ: usize = 3;
    pub const WRITE: usize = 4;
    pub const SET_MODE: usize = 5;
}

#[allow(unused)]
mod opmode {
    pub const SLEEP: usize = 0;
    pub const STANDBY: usize = 1;
    pub const FREQUENCYSYNTHESIZER: usize = 2;
    pub const TRANSMIT: usize = 3;
    pub const RECEIVE: usize = 4;
}

pub struct IsmRadioDriver {  }

impl IsmRadioDriver {
    fn command(command: usize, arg1: usize, arg2: usize) -> TockResult<usize> {
        Ok(syscalls::command(DRIVER_NUMBER, command, arg1, arg2)?)
    }

    pub fn reset(&self) -> TockResult<usize> {
        let r = syscalls::command(
            DRIVER_NUMBER,
            command::RESET,
            0,
            0)?;
        Ok(r)
    }

    pub fn status(&self) -> TockResult<usize> {
        Ok(IsmRadioDriver::command(command::STATUS, 0, 0)?)
    }

    fn set_mode(&self, mode: usize) -> TockResult<usize> {
        let r = syscalls::command(
            DRIVER_NUMBER,
            command::SET_MODE,
            mode,
            0)?;

        Ok(r)
    }

    pub fn sleep(&self) -> TockResult<usize> {
        self.set_mode(opmode::SLEEP)
    }

    pub fn standby(&self) -> TockResult<usize> {
        self.set_mode(opmode::STANDBY)
    }

    pub fn receive(&self) -> TockResult<usize> {
        self.set_mode(opmode::RECEIVE)
    }

    pub fn transmit(&self) -> TockResult<usize> {
        self.set_mode(opmode::TRANSMIT)
    }

    pub fn write(&self, address: u8, value: u8) -> TockResult<usize> {
        IsmRadioDriver::command(command::WRITE, address as usize, value as usize)
    }

    pub fn get_read(&self) -> TockResult<usize> {
        IsmRadioDriver::command(6, 0, 0)
    }

    pub fn sample_fill(&self) -> TockResult<usize> {
        IsmRadioDriver::command(50, 0, 0)
    }
}
