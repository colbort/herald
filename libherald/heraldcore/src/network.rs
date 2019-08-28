use crate::errors::HErr;
use herald_common::{GlobalId, MessageToClient, MessageToServer, RawMsg, UserId};
use lazy_static::*;
use serde::Serialize;
use std::{
    env,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
};

const DEFAULT_PORT: u16 = 8000;
const DEFUALT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

lazy_static! {
    static ref SERVER_ADDR: String = env::var("SERVER_ADDR").unwrap_or_else(|_| format!(
        "{}:{}",
        Ipv4Addr::from(DEFUALT_SERVER_IP_ADDR),
        DEFAULT_PORT
    ));
}

/// Initializes connection with the server.
pub fn open_connection() -> Result<TcpStream, HErr> {
    let socket: SocketAddrV4 = SERVER_ADDR.parse().expect("Invalid server address");
    let mut stream = TcpStream::connect(socket)?;

    login(&mut stream)?;

    Ok(stream)
}

/// Sends `data` such as messages, Registration requests,
/// and metadata to the server.
pub fn send_to_server<T: Serialize>(data: &T, stream: &mut TcpStream) -> Result<(), HErr> {
    let msg_v = serde_cbor::to_vec(data)?;
    stream.write_all(&msg_v.len().to_le_bytes())?;
    stream.write_all(msg_v.as_slice())?;
    Ok(())
}

/// Reads inbound data from the server
pub fn read_from_server(stream: &mut TcpStream) -> Result<(), HErr> {
    stream.set_nonblocking(true)?;

    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;

    stream.set_nonblocking(false)?;

    let len = u64::from_le_bytes(buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;

    let msg = serde_cbor::from_slice(&buf)?;

    match msg {
        MessageToClient::NewMessage { from, text, time } => {
            let body = String::from_utf8(text.to_vec())
                .map_err(|_| HErr::HeraldError("Bad string".into()))?;

            let recipient = crate::config::Config::static_id()?;
            let GlobalId { uid: from, .. } = from;

            crate::message::Messages::add_message(
                from.to_string().as_str(),
                recipient.as_str(),
                body.as_str(),
                Some(time),
            )?;
        }
        _ => unimplemented!(),
    }
    Ok(())
}

/// Registers `user_id` on the server.
pub fn register(user_id: UserId, stream: &mut TcpStream) -> Result<(), HErr> {
    let gid = GlobalId {
        did: 0,
        uid: user_id,
    };

    let msg = MessageToServer::SendMsg {
        to: user_id,
        text: RawMsg::from(""),
    };

    send_to_server(&gid, stream)?;
    send_to_server(&msg, stream)?;

    // Shim code to ack self because the server is ~stupid~
    read_from_server(stream)?;

    Ok(())
}

/// Login
pub fn login(stream: &mut TcpStream) -> Result<(), HErr> {
    let gid = GlobalId {
        did: 0,
        uid: UserId::from(crate::config::Config::static_id()?.as_str())?,
    };

    send_to_server(&gid, stream)
}

/// Sends message to server.
pub fn send_message(to: UserId, text: RawMsg, stream: &mut TcpStream) -> Result<(), HErr> {
    let msg = MessageToServer::SendMsg { to, text };

    send_to_server(&msg, stream)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::process;

    #[test]
    fn register() {
        process::Command::new("cargo")
            .args(&["build", "--bin", "stupid"])
            .current_dir("../../server")
            .output()
            .expect("Failed to start server");

        let mut child = process::Command::new("cargo")
            .args(&["run", "--bin", "stupid"])
            .current_dir("../../server")
            .spawn()
            .expect("Failed to start server");

        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut stream = match super::open_connection() {
            Ok(stream) => stream,
            Err(_) => {
                child.kill().expect("Failed to kill child");
                return;
            }
        };
        if super::register(super::UserId::from("hello").unwrap(), &mut stream).is_err() {
            child.kill().expect("Failed to kill child");
            return;
        }

        child.kill().expect("Failed to kill child");
    }
}
