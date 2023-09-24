use std::{process::Command, io::Error};

use crate::{handle_page_return, default_handle};

pub mod routing;
pub mod handling;

pub fn deploy(stream: &mut std::net::TcpStream) {
    match deploy_inner() {
        Ok(_) => handle_page_return(stream, "200 OK", None, "en/hello.html"),
        Err(e) => default_handle(stream, "404 Page not found", None, e.to_string().as_str()),
    }
}

fn deploy_inner() -> Result<(), Error> {
    Command::new("git")
        .arg("pull")
        .output()?;

    let mut child = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .spawn()?;
    
    build_vue_apps(vec!["pages/vue"])?;

    child.wait()?;
    
    Ok(())
}

fn build_vue_apps(paths: Vec<&str>) -> Result<(), Error> {
    let mut vue_site_path_handle = vec![];
    
    for path in paths {
        Command::new("yarn")
            .current_dir(path)
            .output()?;

        let handle = Command::new("yarn")
            .arg("build")
            .current_dir(path)
            .spawn()?;
        vue_site_path_handle.push(handle);
    }
    for mut child in vue_site_path_handle {
        child.wait()?;
        // replace vue page references
        
        
    }
    Ok(())
}