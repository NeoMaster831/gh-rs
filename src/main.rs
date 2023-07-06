mod wn;
mod lx;

use wn::ext::structs::Proc;

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

    if newproc.get_modules().is_some() {
        println!("Hi Nigger");
        return;
    }
    let mut value = 0;
    match newproc.read::<i32>(0x24BC9A4404C, &mut value) {
        Some(e) => {
            println!("{}", e.to_string());
            return;
        },
        None => println!("{}", value),
    }

    for i in newproc.modules {
        for j in i.szModule {
            if j == 0 {
                break;
            }
            print!("{}", j as u8 as char);
        }
        println!();
    }
}
