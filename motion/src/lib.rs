#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod gcode;
mod gcode_program;
mod home;
mod motion;

pub use crate::{
    gcode_program::GcodeProgram,
    home::{Fault, FaultKind, Home, MoveAxisHome, StartHomingSequence},
    motion::{ControlMode, Motion, MotionParameters},
};
