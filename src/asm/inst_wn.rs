use std::any::Any;

use winapi::{vc::{vadefs::*}};

struct Jmp {
    to: Box<dyn Any> // Operand 1
}

impl Jmp {
    fn p(&self, base: uintptr_t) {

    }
    fn up(barr: &[u8]) -> Self {
        Jmp{to: Box::new(0x1337)}
    }
}
