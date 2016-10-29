#![no_std]
#![allow(non_camel_case_types)]
#![feature(asm)]
#![feature(naked_functions)]

pub use types::*;
pub use builtins::*;
pub use funcs::*;
pub use start::*;
pub use syscall::*;

/// Basic types (not usually system specific)
mod types;
/// Builtins like memcpy and memmove that Redox does not have
mod builtins;
/// Basic functions (not system specific)
mod funcs;
/// Start function and call in to libstd
mod start;
/// Conversion for syscall library (specific to Redox)
mod syscall;
