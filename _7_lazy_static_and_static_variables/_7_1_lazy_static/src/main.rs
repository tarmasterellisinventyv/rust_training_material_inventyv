/*
	Lazy Static
		Lazy static variables are a way to define a static variable that is only initialized when it is first accessed.
		Lazy static variables are useful when you want to define a static variable that is only initialized when it is first accessed.
		Lazy static variables are defined using the lazy_static! macro.
		Lazy static variables are defined using the lazy_static! macro.

		The lazy_static! macro is used to define a lazy static variable.
		The lazy_static! macro is used to define a lazy static variable.
*/

// Importing the lazy_static crate and Mutex
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    // Defining a lazy static variable wrapped in a Mutex for safe mutability
    static ref COUNTER: Mutex<u32> = Mutex::new(0);
}

fn main() {
    // Lock the mutex to access the counter safely
    {
        let mut counter = COUNTER.lock().unwrap(); // Lock the mutex
        *counter += 1; // Increment the counter
        println!("Counter: {}", *counter); // Print the current counter value
    }

    {
        let mut counter = COUNTER.lock().unwrap(); // Lock the mutex again
        *counter += 1; // Increment the counter
        println!("Counter: {}", *counter); // Print the current counter value
    }
}