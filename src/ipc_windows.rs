use crate::{discord_ipc::DiscordIpc, error::Error};
use serde_json::json;
use std::{
    io::{Read, Write},
    path::PathBuf,
};
use windows_named_pipe::PipeStream;

type Result<T> = std::result::Result<T, crate::error::Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct DiscordIpcClient {
    pub client_id: String,
    pub connected: bool,
    pub socket: Option<PipeStream>,
}

impl DiscordIpc for DiscordIpcClient {
    fn connect_ipc(&mut self) -> Result<()> {
        for i in 0..10 {
            let path = PathBuf::from(format!(r"\\?\pipe\discord-ipc-{}", i));

            match PipeStream::connect(&path) {
                Ok(socket) => {
                    self.socket = Some(socket);
                    return Ok(());
                }
                Err(_) => continue,
            }
        }

        Err(Error::ConnectionFailure(
            "Couldn't connect to the Discord IPC socket".into(),
        ))
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

    fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
