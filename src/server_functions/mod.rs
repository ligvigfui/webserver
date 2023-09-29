use std::{process::Command, io::Error};

use crate::*;

pub mod routing;
pub mod handling;

pub fn code404(stream: &mut std::net::TcpStream, request: Request) {
    println!("404 Page not found");
    handle_page_return(stream, "404 NOT FOUND", None,
    &(format!("{}/404.html", request.get_header("Accept-Language").unwrap())));
}

pub fn deploy(stream: &mut std::net::TcpStream) {
    match deploy_inner() {
        Ok(_) => handle_page_return(stream, "200 OK", None, "en/hello.html"),
        Err(e) => default_handle(stream, "400 Bad Request", None, e.to_string().as_str()),
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

    Command::new("systemctl")
        .arg("restart")
        .arg("neptunCRF")
        .output()?;
    
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
    }
    Ok(())
}