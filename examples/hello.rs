//! This example isn't faster, smaller, simpler, more efficient, more portable,
//! or more desireable than regular hello world in any practical way. It just
//! demonstrates the API of this crate.

#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use std::io::{self, stdout, Write};
use std::mem::ManuallyDrop;
#[cfg(not(windows))]
use unsafe_io::os::rsix::{AsRawFd, FromRawFd};
use unsafe_io::{AsUnsafeFile, AsUnsafeHandle, FromUnsafeFile};
#[cfg(windows)]
use {std::os::windows::io::FromRawHandle, unsafe_io::os::windows::AsRawHandleOrSocket};

fn main() -> io::Result<()> {
    let stdout = stdout();
    let stdout = stdout.lock();

    // Obtain an `UnsafeWriteable` and use it to write.
    writeln!(
        unsafe { stdout.as_unsafe_handle().as_writeable() },
        "hello, world"
    )?;

    // Obtain an `UnsafeFile` and use it to construct a temporary manually-drop
    // `File` to write.
    writeln!(stdout.as_file_view(), "hello, world")?;

    // Similar, but do it manually.
    writeln!(
        ManuallyDrop::new(unsafe { std::fs::File::from_unsafe_file(stdout.as_unsafe_file()) }),
        "hello, world"
    )?;

    // Similar, but gratuitously pass stdout through `from_filelike`.
    writeln!(
        ManuallyDrop::new(std::fs::File::from_filelike(unsafe {
            std::fs::File::from_unsafe_file(stdout.as_unsafe_file())
        })),
        "hello, world"
    )?;

    // Similar, but even more gratuitously pass stdout through `as_file_view`
    // and `from_filelike`.
    writeln!(
        ManuallyDrop::new(std::fs::File::from_filelike(unsafe {
            std::fs::File::from_unsafe_file(stdout.as_file_view().as_unsafe_file())
        })),
        "hello, world"
    )?;

    // Similar, but use the Posix-ish-specific type.
    #[cfg(not(windows))]
    writeln!(
        ManuallyDrop::new(unsafe {
            std::fs::File::from_raw_fd(stdout.as_unsafe_handle().as_raw_fd())
        }),
        "hello, world"
    )?;

    // Similar, but use the Windows-specific type.
    #[cfg(windows)]
    writeln!(
        ManuallyDrop::new(unsafe {
            std::fs::File::from_raw_handle(
                stdout
                    .as_unsafe_handle()
                    .as_raw_handle_or_socket()
                    .as_unowned_raw_handle()
                    .unwrap(),
            )
        }),
        "hello, world"
    )?;

    Ok(())
}
