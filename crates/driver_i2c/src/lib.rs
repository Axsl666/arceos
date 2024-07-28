#![no_std]

mod hal;
mod time;
mod pac;

// pub type Result<T = ()> = core::result::Result<T, Error>;

const MIO0_BASE: u64 = 0xffff_0000_2801_4000;


