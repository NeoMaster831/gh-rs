mod wn;
mod lx;
mod asm;

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

    if newproc.get_modules().is_some() {
        println!("Hi Nigger");
        return;
    }
    let mut value = 0;
    match newproc.read::<i32>(0x1CB07E4436C, &mut value) {
        Some(e) => {
            println!("{}", e.to_string());
            return;
        },
        None => println!("{}", value),
    }
    match newproc.write::<i32>(0x1CB07E4436C, &mut 1337) {
        Some(e) => {
            println!("{}", e.to_string());
            return;
        }
        None => {
            newproc.read::<i32>(0x1CB07E4436C, &mut value);
            println!("{}", value);
        }
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

#[cfg(target_os = "linux")]
fn main() {
    println!("Linux is not support at the moment");
}