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
    unsafe {               // Be careful of ioctl()
      led_set_all(         // Set the LEDs for...
          fd.as_raw_fd(),  // LED Device
          1                // LED 1 (Bit 0) turns On
      ).unwrap();          // Halt on Error
    }  // Equivalent to ioctl(fd, ULEDIOC_SETALL, 1)

    // Wait 2 seconds
    sleep(2);

    // Flip LED 1 to Off: ioctl(fd, ULEDIOC_SETALL, 0)
    unsafe { led_set_all(fd.as_raw_fd(), 0).unwrap(); }

    // Serialize and Deserialize JSON
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

    // Run 4 Async Functions in the Background
    test_async();
}

// Run 4 Async Functions in the Background
// By creating One New NuttX Thread
// Based on https://tokio.rs/tokio/topics/bridging
fn test_async() {

    // Create a Multi-Threaded Scheduler
    // Containing One New NuttX Thread
    let runtime = tokio::runtime::Builder
        ::new_multi_thread() // Multi-Threaded Scheduler
        .worker_threads(1)   // With One New NuttX Thread for our Scheduler
        .enable_all() // Enable the I/O and Time Functions
        .build()      // Create the Multi-Threaded Scheduler
        .unwrap();    // Halt on Error
  
    // Create 4 Async Functions
    // Remember their Async Handles
    let mut handles = Vec::with_capacity(4);
    for i in 0..4 {
        handles.push(            // Remember the Async Handles
            runtime.spawn(       // Start in the Background
                my_bg_task(i))); // Our Async Function
    }
  
    // Pretend to be busy while Async Functions execute (in the background)
    // We wait 750 milliseconds
    std::thread::sleep(
          tokio::time::Duration::from_millis(750));
    println!("Finished time-consuming task.");
  
    // Wait for All Async Functions to complete
    for handle in handles {
        runtime
            .block_on(handle)  // Wait for One Async Function to complete
            .unwrap();
    }
  }
  
// Our Async Function that runs in the background...
// If i=0: Sleep for 1000 ms
// If i=1: Sleep for  950 ms
// If i=2: Sleep for  900 ms
// If i=3: Sleep for  850 ms
async fn my_bg_task(i: u64) {
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms.", i, millis);
    tokio::time::sleep(
        tokio::time::Duration::from_millis(millis)
    ).await;  // Wait for sleep to complete
    println!("Task {} stopping.", i);
}

// Needed by Tokio Multi-Threaded Scheduler
#[no_mangle]
pub extern "C" fn pthread_set_name_np() {}
