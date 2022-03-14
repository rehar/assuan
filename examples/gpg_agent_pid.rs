use assuan::context::Context;
use gpg_error::{Error, Result};
use std::{process::Command, path::Path};
fn get_agent_socket() -> Result<String> {
    let out = Command::new("sh")
        .arg("-c")
        .arg("gpgconf --list-dirs")
        .output()
        .expect("failed to execute process");
    let out = match String::from_utf8(out.stdout) {
        Ok(s) => s,
        Err(e) => return Err(Error::INV_RESPONSE),
    };
    let mut socket = None;
    for line in out.lines() {
        let elements: Vec<&str> = line.trim().split(":").collect();
        if elements.len() >= 2 && elements[0].eq_ignore_ascii_case("agent-socket"){
            socket = Some(elements[1]);
            break;
        }
    }

    if let Some(sock) = socket {    
        Ok(sock.to_string())
    } else {
        Err(Error::ENOTSOCK)
    }        
}
fn main() -> Result<()> {

    let socket = get_agent_socket()?;
    let mut ctx = Context::new()?;
    ctx.socket_connect(Path::new(&socket))?;
    ctx.transact("GETINFO pid", |data| {
        
        //panic!("fdfdf");
        println!("gpg-agent PID is {}.", String::from_utf8(data.to_vec()).unwrap());
        Ok(())
    }, |_|{Ok(())}, |_|{Ok(())})?;
    
    Ok(())
}
