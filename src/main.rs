mod wn;
mod lx;
mod asm;
mod utils;

#[cfg(target_os = "windows")]
use wn::ext::structs::Proc;

#[cfg(target_os = "windows")]
fn main() {
    use std::time::Instant;

    use crate::utils::alg::pat_fft;


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

    const SIZE: usize = 3;
    let pat: [u8; SIZE] = [ 0x0a; SIZE ];
    let range: [usize; 2] = [0x1C000000000, 0x1C500000000];

    let mut a : Vec<u8> = vec![];
    for _ in 0..SIZE {
        a.push(b'x');
    }
    let start_time = Instant::now();
    match newproc.pat_scan(&pat, a.as_slice(), range[0], range[1], "dump") {
        Some(wh) => println!("{:x}", wh),
        None => println!("error"),
    };
    println!("Elapsed time: {:?}", Instant::now().duration_since(start_time));

    return;
}

#[cfg(target_os = "linux")]
fn main() {
    println!("Linux is not support at the moment");
}