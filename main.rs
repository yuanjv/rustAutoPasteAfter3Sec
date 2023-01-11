use std::io;
use std::{thread, time};
use rsautogui::keyboard::{self};

fn main() {
    while true{
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("paste here:");
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("ERROR>>>failed to read line");
        thread::sleep(time::Duration::from_secs(3));
        keyboard::typewrite(&input);
    }
}
