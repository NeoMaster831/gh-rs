use std::{io::*, mem::*};

use winapi::{vc::{vadefs::*}, um::{memoryapi::*}, ctypes::*, shared::{ntdef::*}};

use super::structs::Proc;

impl Proc {

    pub fn read<T>(&mut self, wh: uintptr_t, storeat: *mut T) -> Option<Error> {
        if self.handle == unsafe { zeroed() } {
            Some(Error::new(std::io::ErrorKind::InvalidData, "Handle is invalid"));
        }

        let res = unsafe {
            ReadProcessMemory(
                self.handle, 
                wh as *const c_void, 
                storeat as *mut c_void, 
                size_of::<T>(), 
                NULL as *mut usize,
            )
        };
        if res == 0 {
            Some(Error::new(ErrorKind::Other, "Couldn't read memory"));
        }
        None
    }

}