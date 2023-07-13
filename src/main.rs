mod wn;
mod lx;
mod asm;
mod utils;

#[cfg(target_os = "windows")]
use wn::ext::structs::Proc;

#[cfg(target_os = "windows")]
fn main() {

    let mut newproc = Proc::new();
    match newproc.get_pid("Notepad.exe") {
        Some(e) => {
            println!("{}", e.to_string());
            return;
        },
        None => println!("{}", newproc.pid),
    }

    match newproc.open() {
        Some(e) => {
            println!("{}", e.to_string());
            return;
        },
        None => println!("{:p}", newproc.handle),
    }
    
    match newproc._pat_dump("o ".as_bytes(), "x?".as_bytes(), 0x0, "Dance Number Wo Tomo Ni".as_bytes()) {
        Some(ptr) => println!("{:x}", ptr),
        None => println!("error"),
    };

}

#[cfg(target_os = "linux")]
fn main() {
    println!("Linux is not support at the moment");
}