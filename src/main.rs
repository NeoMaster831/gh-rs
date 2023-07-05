mod wn;
mod lx;

use wn::ext::structs::Proc;

fn main() {
    let mut newproc = Proc::new();
    match newproc.get_pid("Notepad.exe") {
        Some(_) => println!("None of them found"),
        None => println!("{}", newproc.pid),
    }
}
