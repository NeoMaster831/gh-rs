use std::{mem::*};

use winapi::{vc::{vadefs::*}, um::{memoryapi::*, errhandlingapi::GetLastError}, shared::{ntdef::*, basetsd::*, minwindef::*}};

use super::structs::Proc;

impl Proc {

    // read mem. If error occurs, return last error. check msdn.
    pub fn read<T>(&mut self, wh: uintptr_t, storeat: *mut T) -> Option<DWORD> {

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
            let s = unsafe { GetLastError() };
            return Some(s);
        }
        None
        
    }

    // write mem. If error occurs, return last error. check msdn.
    pub fn write<T>(&mut self, wh: uintptr_t, what: *mut T) -> Option<DWORD> {

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
            let s = unsafe { GetLastError() };
            return Some(s);
        }
        None

    }

    // TODO: add pattern search
    // TODO: add patch

}