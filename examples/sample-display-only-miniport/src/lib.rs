// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! # Sample WDDM display-only miniport
//!
//! Exists to prove the `display` API subset all the way to the linker: it is
//! not enough that bindgen emits `DxgkInitializeDisplayOnlyDriver`, the symbol
//! also has to resolve against `displib.lib`.
//!
//! It is deliberately the smallest thing that can fail for the right reason.
//! The initialization data is zeroed and the callbacks are null, so this driver
//! would be rejected at runtime -- but runtime is not what this proves. If the
//! `#[link(name = "displib")]` directive were missing or wrong, this crate
//! would not link, and that is the question being asked.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;
use wdk_sys::{
    DRIVER_OBJECT, KMDDOD_INITIALIZATION_DATA, NTSTATUS, PCUNICODE_STRING, PUNICODE_STRING,
    display::DxgkInitializeDisplayOnlyDriver,
};

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

/// `DriverEntry` for a display-only miniport.
///
/// # Safety
/// Dereferences raw pointers handed over by the kernel.
#[cfg_attr(not(test), unsafe(export_name = "DriverEntry"))]
pub unsafe extern "system" fn driver_entry(
    driver: *mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    // SAFETY: KMDDOD_INITIALIZATION_DATA is a plain C struct of scalars and
    // function pointers, for which an all-zero bit pattern is valid.
    let mut init: KMDDOD_INITIALIZATION_DATA = unsafe { core::mem::zeroed() };
    init.Version = 0;

    // SAFETY: `driver` and `registry_path` come from the kernel and are valid
    // for the duration of this call; `init` is a live local.
    unsafe {
        DxgkInitializeDisplayOnlyDriver(
            driver,
            registry_path.cast_mut().cast::<_>() as PUNICODE_STRING,
            &raw mut init,
        )
    }
}
