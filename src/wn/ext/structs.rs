use std::{mem::*, io::*};

use winapi::{um::{tlhelp32::*, winnt::*}, shared::minwindef::*};

// Define some Objects

pub struct Proc {
    pub handle: HANDLE,
    pub pid: DWORD,
}

impl Proc {

    // Creates new instance.
    pub fn new() -> Self {
        Self { 
            handle: unsafe { zeroed() }, 
            pid: 0 
        }
    }

    // Creates Proc object with PID. the process name is required.
    pub fn get_pid(&mut self, name: &str) -> Option<Error> {

        let snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0) };
        let mut entry: PROCESSENTRY32 = unsafe { zeroed() };
        entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
            
        let mut valid = unsafe { Process32First(snap, &mut entry) };
        while valid != 0 {
            let proc_name = entry.szExeFile;
            let mut found = true;
            for i in (0 as usize)..name.len() {
                if proc_name[i] as u8 != name.as_bytes()[i] {
                    found = false;
                    break;
                }
            }
            if found {
                self.pid = entry.th32ProcessID;
                return None;
            }
            valid = unsafe { Process32Next(snap, &mut entry) };
        }

        Some(Error::new(ErrorKind::NotFound, "Process not found."))

    }
}