//cargo add chrono

use chrono::Local;
use chrono::Utc;

pub fn get_local_time()-> String {
    return Local::now().to_string();
}

pub fn get_utc_time()-> String {
    return Utc::now().to_string();
}