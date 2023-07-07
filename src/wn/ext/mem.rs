use std::{io::*, mem::*};

use winapi::{vc::{vadefs::*}, um::{memoryapi::*}, shared::{ntdef::*, basetsd::*, minwindef::*}};

use super::structs::Proc;

impl Proc {

    pub fn read<T>(&mut self, wh: uintptr_t, storeat: *mut T) -> Option<Error> {

        if self.handle == unsafe { zeroed() } {
            return Some(Error::new(std::io::ErrorKind::InvalidData, "Handle is invalid"));
        }

        let res = unsafe {
            ReadProcessMemory(
                self.handle, 
                wh as LPCVOID, 
                storeat as LPVOID, 
                size_of::<T>() as SIZE_T, 
                NULL as *mut SIZE_T,
            )
        };

        if res == 0 {
            return Some(Error::new(ErrorKind::Other, "Couldn't read memory"));
        }
        None
        
    }

    pub fn write<T>(&mut self, wh: uintptr_t, what: *mut T) -> Option<Error> {
        
        if self.handle == unsafe { zeroed() } {
            return Some(Error::new(std::io::ErrorKind::InvalidData, "Handle is invalid"));
        }

        let res = unsafe {
            WriteProcessMemory(
                self.handle, 
                wh as LPVOID, 
                what as LPCVOID,
                size_of::<T>() as SIZE_T, 
                NULL as *mut SIZE_T
            )
        };

        if res == 0 {
            return Some(Error::new(ErrorKind::Other, "Couldn't write memory"));
        }
        None

    }

    // TODO: add pattern search
    
}