//! Windows Named Pipe client wrapper

#![cfg(windows)]

use std::io::{self, Read, Write};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_NONE, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, ReadFile, WriteFile, FlushFileBuffers,
};
use windows::Win32::System::Pipes::{PIPE_READMODE_BYTE, SetNamedPipeHandleState};

pub struct NamedPipeStream {
    handle: HANDLE,
}

impl Read for NamedPipeStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0u32;
        unsafe {
            ReadFile(
                self.handle,
                Some(buf),
                Some(&mut bytes_read),
                None,
            ).map_err(|e| io::Error::from_raw_os_error(e.code().0))?;
        }
        Ok(bytes_read as usize)
    }
}

impl Write for NamedPipeStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0u32;
        unsafe {
            WriteFile(
                self.handle,
                Some(buf),
                Some(&mut bytes_written),
                None,
            ).map_err(|e| io::Error::from_raw_os_error(e.code().0))?;
        }
        Ok(bytes_written as usize)
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe {
            FlushFileBuffers(self.handle)
                .map_err(|e| io::Error::from_raw_os_error(e.code().0))?;
        }
        Ok(())
    }
}

impl Drop for NamedPipeStream {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

pub fn connect_named_pipe(path: &str) -> io::Result<NamedPipeStream> {
    use windows::core::PCWSTR;

    let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        let handle = CreateFileW(
            PCWSTR(wide_path.as_ptr()),
            (FILE_GENERIC_READ.0 | FILE_GENERIC_WRITE.0),
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        ).map_err(|e| io::Error::from_raw_os_error(e.code().0))?;

        let mut mode = PIPE_READMODE_BYTE;
        SetNamedPipeHandleState(
            handle,
            Some(&mut mode),
            None,
            None,
        ).map_err(|e| io::Error::from_raw_os_error(e.code().0))?;

        Ok(NamedPipeStream { handle })
    }
}
