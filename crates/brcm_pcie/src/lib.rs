//! Device filesystem used by [ArceOS](https://github.com/rcore-os/arceos).
//!
//! The implementation is based on [`axfs_vfs`].
#![no_std]
#![feature(const_ptr_as_ref)]
#![feature(const_option)]
#![feature(const_nonnull_new)]

mod bcm2711;
mod interrupt;
mod consts;



pub use bcm2711::*;
/// reset controller

/// sets bit 1 of [pcie->base+0x9210] to val
pub trait BCM2711Hal {
    fn sleep(ms:core::time::Duration);
}