use std::io;
use std::{thread, time};
use rsautogui::keyboard::{self,Key};

fn main() {
    let mut list = vec![];
    let mut input=String::new();
    let lowerCasesStr="`1234567890-=qwertyuiop[]\\asdfghjkl;'zxcvbnm,./";
    let lowerCases: Vec<_>=lowerCasesStr.chars().collect();
    //loop
    while true{
        //clear terminal
        print!("{}[2J", 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("paste here  >start new line and type &*( to start<:");
        //
        while &input != "&*(\n"{
            //BUG return doesn't work
            //get every single line and append them to a array

            //list[0]=""
            list.insert(list.len(),input);
            //read_line append string, need manuel clean up
            input="".to_string();
            io::stdin().read_line(&mut input).expect("ERROR>>>failed to read line");
        }
        //wait
        thread::sleep(time::Duration::from_secs(3));
        //type every single 
        for l in &list{
            //BUG uppercases doesn't work
            //do chars
            let chars: Vec<_> = l.chars().collect();
            for i in chars{
                let mut notHappened=true;
                for n in &lowerCases{
                    if i==*n{
                        keyboard::typewrite(&i.to_string());
                        notHappened=false;
                    }  
                }
                if notHappened{
                    keyboard::key_down(Key::Shift);
                    keyboard::typewrite(&i.to_string());
                    keyboard::key_up(Key::Shift);
                }

            }
            //manage list[0]=""
            if l!=""{
                //BUG always type a "a" when return is needed
                keyboard::key_tap(Key::Backspace);
                keyboard::key_tap(Key::Return);
            }
        }
        //reset
        list.clear();
        input="".to_string();
    }
}
