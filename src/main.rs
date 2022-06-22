mod multiLineState;

use std::arch::x86_64::_mm_pause;
use std::thread::park_timeout;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use multiLineState::MultiLineState;

fn main() -> Result<()> {
    // println!("result start {}", reverse("testing").unwrap());
    // park_timeout(1000ms);
    let mut paste: bool = false;
    let mut paste_state = MultiLineState::new();
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                
                rl.add_history_entry(line.as_str());
                if paste {
                    //add the line to the multiline state
                    paste_state.addRecord(line.to_string());
                }


                // let string = "line one
                // line two";
                // rl.add_history_entry(string);
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                paste = !paste;
                println!("CTRL-D: Paste mode is {}", paste);


                if !paste && paste_state.entries() > 0 {
                    rl.add_history_entry(paste_state.resultString());
                }
                //clear the vec
                if paste == false{
                    paste_state.cleanse();
                }
                continue
                // break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt")
}