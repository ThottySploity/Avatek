// Copyright (c) 2024 ThottySploity

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::utilities::Utilities;

use anyhow::Result;
use windows_sys::Win32::Storage::FileSystem::CreateFileW;
use windows_sys::Win32::Storage::FileSystem::ReadFile;
use windows_sys::Win32::Storage::FileSystem::WriteFile;

use windows_sys::Win32::System::Pipes::PeekNamedPipe;
use windows_sys::Win32::System::Pipes::CreateNamedPipeW;

use std::ptr::null_mut;
use std::ffi::c_void;

pub struct Pipes;

impl Pipes {

    pub fn connect(pipename: String) -> Result<isize> {
        unsafe {
            // Connecting to a named pipe using CreateFile
            Ok(CreateFileW(
                Utilities::pointer_to_constant_wide(&pipename).as_ptr(),
                2147483648u32 | 1073741824u32,  // GENERIC_READ | GENERIC_WRITE
                0,
                null_mut(),
                3u32,
                3u32,
                null_mut(),
            ) as isize)
        }
    }

    pub fn create(pipename: String) -> Result<isize> {
        unsafe {
            // Creating named pipe allowing remote connections and PIPE_UNLIMITED_INSTANCES (only limited by system resources)
            Ok(CreateNamedPipeW(
                Utilities::pointer_to_constant_wide(&pipename).as_ptr(),
                0x00000003,     // ACCESS_DUPLEX
                0x00000000,     // PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_ACCEPT_REMOTE_CLIENTS
                255,            // Amount of Client connections
                0,              
                0,
                0,
                null_mut(),
            ) as isize)
        }
    }

    pub fn peek(handle: isize) -> u32 {
        unsafe {
            // Peeking into a pipe, this is to say if there are bytes in the pipe
            let mut bytes_available: u32 = 0;

            PeekNamedPipe(
                handle as *mut c_void,
                null_mut(),
                0,
                null_mut(),
                &mut bytes_available,
                0 as *mut u32,
            );
            bytes_available
        }
    }

    pub fn read(handle: isize, len: u32) -> Vec<u8> {
        unsafe {
            // Reading the information out of the pipe
            let mut buffer = vec![0u8; len as usize].into_boxed_slice();
            let mut bytes_read: u32 = 0;

            ReadFile(
                handle as *mut c_void,
                buffer.as_mut_ptr() as *mut _,
                len as u32,
                &mut bytes_read,
                null_mut(),
            );
            buffer.to_vec()
        }
    }

    pub fn write(handle: isize, data: Vec<u8>) {
        unsafe {
            // Writing information to the pipe

            WriteFile(
                handle as *mut c_void,
                data.as_ptr(),
                data.len() as u32,
                null_mut(),
                null_mut(),
            );
        }
    }
}