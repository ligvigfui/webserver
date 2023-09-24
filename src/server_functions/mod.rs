use std::process::Command;

pub mod routing;
pub mod handling;

pub fn deploy() {
    // run yarn build
    let mut child = Command::new("yarn")
        .arg("build")
        .current_dir("pages/vue")
        .spawn()
        .expect("Failed to run yarn build");

    let mut child2 = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()
        .expect("Failed to run cargo run");

    child.wait().expect("yarn build failed");
    child2.wait().expect("cargo build failed");
    
    println!("yarn build finished")
    // replace vue page references
    
}