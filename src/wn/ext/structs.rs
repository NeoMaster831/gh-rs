use std::{mem::*, io::*};

use winapi::{um::{tlhelp32::*, winnt::*, processthreadsapi::*}, shared::minwindef::*};

// Define some Objects

pub struct Proc {
    pub handle: HANDLE,
    pub pid: DWORD,
    pub modules: Vec<MODULEENTRY32>
}

impl Proc {

    // Creates new instance.
    pub fn new() -> Self {
        Self { 
            handle: unsafe { zeroed() }, 
            pid: 0,
            modules: vec![]
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

    // Open it, and get modules info.
    pub fn open(&mut self) -> Option<Error> {
        if self.pid == 0 {
            return Some(Error::new(ErrorKind::InvalidData, "Process' PID is invalid!"))
        }
        self.handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, self.pid) };
        if self.handle == unsafe { zeroed() } {
            return Some(Error::new(ErrorKind::Interrupted, "The procedure got interrupted. Maybe ac?"));
        } else {
            return None;
        }
    }

    pub fn get_modules(&mut self) -> Option<Error> {

        if self.pid == 0 {
            return Some(Error::new(ErrorKind::InvalidData, "Process' PID is invalid!"))
        } 

        self.modules = vec![];
        let hd = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.pid) };
        let mut entry: MODULEENTRY32 = unsafe { zeroed() };
        entry.dwSize = size_of::<MODULEENTRY32>() as u32;

        let mut valid = unsafe { Module32First(hd, &mut entry) };
        while valid != 0 {
            self.modules.push(entry);
            valid = unsafe { Module32Next(hd, &mut entry) };
        }
        None
        
    }

}