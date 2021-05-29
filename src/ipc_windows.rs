use crate::discord_ipc::DiscordIpc;
use serde_json::json;
use std::{
    error::Error,
    io::{Read, Write},
    path::PathBuf,
};
use windows_named_pipe::PipeStream;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[allow(dead_code)]
pub struct DiscordIpcClient {
    pub connected: bool,
    pub socket: Option<PipeStream>,
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        let mut path: PathBuf = PathBuf::new();

        for i in 0..10 {
            path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", i));

            if !path.exists() {
                continue;
            } else {
                break;
            }
        }

        let socket = PipeStream::connect(path).expect("Failed to connect to socket");
        self.socket = Some(socket);

        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<()> {
        let socket = self.socket.as_mut().unwrap();

        socket.write_all(data)?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<()> {
        let socket = self.socket.as_mut().unwrap();

        socket.read_exact(buffer)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let data = json!({});
        if self.send(data, 2).is_ok() {}

        self.socket.as_mut().unwrap().flush()?;

        Ok(())
    }
}
