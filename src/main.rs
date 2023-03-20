use std::env::args;
use std::fs::OpenOptions;
use std::io::{
    self,
    Read,
    Write, BufRead,
};
use std::process::exit;

const XED: &str = r#"
    Welcome to the extended ed editor (xed).

    By typing: 'a' or 'append' you can enter EDIT mode.
    To exit the editor type: 'q' or 'quit'.
"#;

const HELP: &str = r#"
    extended ed (xed)
    Usage: 
        xed [file]  Open a file on xed
        xed         Open xed w/o a file
    
    Commands:
        a, append       Enter EDIT mode and append when the ESC key is struck
        h, help         Print this message
        p, print        Print the contents of the file buffer
        q, quit         Exit the editor
"#;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Error: No input files");
        exit(1);
    }
    if args.len() > 2 {
        println!("Error: More files met");
        exit(1);
    }
    let mut command_buffer = String::new();
    let mut append_buffer = String::new();
    let mut file_buffer = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&args[1])
        .unwrap();
    file.read_to_string(&mut file_buffer)
        .expect("Error: Could not read the file contents to the file buffer");
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    loop {
        print!("xed: ");
        stdout_lock.flush().unwrap();
        match stdin_lock.read_line(&mut command_buffer) {
            Ok(n) => {
                let command = command_buffer.replace("\n", "");
                match command.as_str() {
                    "a"|"append"    => {
                        loop {
                            print!("+ ");
                            stdout_lock.flush().unwrap();
                            stdin_lock.read_line(&mut append_buffer)
                                .expect("Could not read line to append buffer");
                            if cfg!(debug_assertions) {
                                dbg!(&append_buffer);
                            }
                            if append_buffer.ends_with("\u{1b}\n") {
                                append_buffer = append_buffer.replace("\u{1b}\n", "");
                                file_buffer.push_str(&append_buffer);
                                break;
                            }
                        }
                    },
                    "h"|"help"      => {
                        println!("{}", HELP);
                    },
                    "p"|"print"     => {
                        println!("{}", file_buffer);
                    },
                    "q"|"quit"      => {
                        println!("Exiting");
                        exit(1);
                    },
                    "w"|"write"     => {
                        if cfg!(debug_assertions) {
                            println!("Writing to {:?}", file);
                        } else {
                            println!("Writing {} byte(s) to {}", file_buffer.len(), &args[1]);
                        }
                        file.write_all(&file_buffer.as_bytes());
                    },
                    _               => {
                        println!("Error: Unrecognized command {}", command);
                    },
                }
            }
            Err(err) => println!("Error: {}", err),
        }
        command_buffer.clear();
    }
}
