use std::any::Any;

use winapi::{vc::{vadefs::*}};

struct Jmp {
    to: dyn Any // Operand 1
}

impl Jmp {
    fn translate(&self, base: uintptr_t) {
        
    }
}