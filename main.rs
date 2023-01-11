use std::io;
use std::{thread, time};
use rsautogui::keyboard::{self,Key};

fn main() {
    let mut list = vec![];
    let mut input=String::new();
    while true{
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("paste here >>>type &*( to start<<<:");

        while &input != "&*(\n"{
            list.insert(list.len(),input);
            input="".to_string();
            io::stdin().read_line(&mut input).expect("ERROR>>>failed to read line");
        }
        thread::sleep(time::Duration::from_secs(3));
        for l in &list{
            keyboard::typewrite(&l);
            if l!=""{
                keyboard::key_down(Key::Return);
            }
        }
        list.clear();
        input="".to_string();
    }
}
