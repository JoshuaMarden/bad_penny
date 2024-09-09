// src/main.rs

mod ingestion {
    pub mod ons_api;  // Import the ons_api.rs file as a module
}

fn main() {
    println!("Main script executed");

    // Call the `run()` function from `ons_api.rs`
    if let Err(e) = ingestion::ons_api::run() {
        println!("Error executing ons_api: {}", e);
    }
}