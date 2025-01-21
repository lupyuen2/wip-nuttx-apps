extern crate serde;
extern crate serde_json;

use std::os::fd::AsRawFd;
use serde::{Deserialize, Serialize};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    ioctl_write_int_bad,
    unistd::sleep
};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

// Function hello_rust_cargo without mangling
#[no_mangle]
pub extern "C" fn hello_rust_cargo_main() {

    // Open the LED Device for NuttX
    let fd = open(        // Equivalent to NuttX open()
        "/dev/userleds",  // LED Device
        OFlag::O_WRONLY,  // Write Only
        Mode::empty()     // No Modes
    ).unwrap();           // Halt on Error

    // Define the ioctl() function for Flipping LEDs
    const ULEDIOC_SETALL: i32 = 0x1d03;  // ioctl() Command
    ioctl_write_int_bad!(  // ioctl() will write One Int Value (LED Bit State)
        led_set_all,       // Name of our New Function
        ULEDIOC_SETALL     // ioctl() Command to send
    );

    // Flip LED 1 to On
    unsafe {             // Be careful of ioctl()
    led_set_all(         // Set the LEDs for...
        fd.as_raw_fd(),  // LED Device
        1                // LED 1 (Bit 0) turns On
    ).unwrap();          // Halt on Error
    }  // Equivalent to ioctl(fd, ULEDIOC_SETALL, 1)

    // Wait 2 seconds
    sleep(2);

    // Flip LED 1 to Off: ioctl(fd, ULEDIOC_SETALL, 0)
    unsafe { led_set_all(fd.as_raw_fd(), 0).unwrap(); }

    // Print hello world to stdout

    let john = Person {
        name: "John".to_string(),
        age: 30,
    };

    let json_str = serde_json::to_string(&john).unwrap();
    println!("{}", json_str);

    let jane = Person {
        name: "Jane".to_string(),
        age: 25,
    };

    let json_str_jane = serde_json::to_string(&jane).unwrap();
    println!("{}", json_str_jane);

    let json_data = r#"
        {
            "name": "Alice",
            "age": 28
        }"#;

    let alice: Person = serde_json::from_str(json_data).unwrap();
    println!("Deserialized: {} is {} years old", alice.name, alice.age);

    let pretty_json_str = serde_json::to_string_pretty(&alice).unwrap();
    println!("Pretty JSON:\n{}", pretty_json_str);

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world from tokio!");
        });

    println!("Looping Forever...");
    loop {
        // Do nothing
    }
}
