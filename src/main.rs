// This is always needed to avoid build error in reference to 'app_main)
// "undefined reference to `app_main' (xtensa-esp32-elf/bin/ld: esp-idf/freertos/libfreertos.a)""
#[allow(unused)]
use esp_idf_sys::{self};

// The most basic example
fn main()  {
    println!("Hello from an ESP32");
}