
use std::sync::{ Mutex };

lazy_static! {
    static ref X_POSITION: Mutex<i32> = Mutex::new(8);
    static ref Y_POSITION: Mutex<i32> = Mutex::new(48);
}

