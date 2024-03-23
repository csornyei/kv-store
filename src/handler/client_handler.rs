use crate::commands::Command;
use crate::data::DataManager;
use crate::persistence::PersistenceType;
use crate::session::Session;
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct ClientHandler {
    socket: TcpStream,
    data: Arc<Mutex<DataManager>>,
}

impl ClientHandler {
    pub fn new(socket: TcpStream, data: Arc<Mutex<DataManager>>) -> Self {
        Self { socket, data }
    }

    fn parse_line(&self, buf: [u8; 1024], line_length: usize) -> String {
        let line = String::from_utf8_lossy(&buf[..line_length]);
        let line = line.trim();
        let line = String::from(line);
        line
    }

    fn split_line(&self, line: String) -> Vec<String> {
        line.split_inclusive(";")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }

    fn continue_command(&self, session: &mut Session, mut commands: Vec<String>) -> Vec<String> {
        if session.incomplete_command.len() > 0 {
            if commands.len() > 0 {
                let first_command = commands.remove(0);
                let real_first_command =
                    format!("{} {}", session.incomplete_command, first_command);
                commands.insert(0, real_first_command);
                session.incomplete_command = "".to_string();
            }
        }
        commands
    }

    async fn execute_command(
        &self,
        data: Arc<Mutex<DataManager>>,
        session: Session,
        command: Command,
    ) -> Result<(String, Session), String> {
        let mut data = data.lock().await;
        let result = data.handle_command(command, session)?;
        if data.persistence.get_type() == PersistenceType::JsonFile {
            match data.save_to_file() {
                Ok(_) => Ok(result),
                Err(e) => Err(e),
            }
        } else {
            Ok(result)
        }
    }

    fn handle_command_result(
        &self,
        result: Result<(String, Session), String>,
        session: &mut Session,
    ) -> String {
        match result {
            Ok((response, new_session)) => {
                session.update(new_session);
                response.to_string()
            }
            Err(e) => e.to_string(),
        }
    }

    async fn write_results(&mut self, results: Vec<String>) {
        let mut results_string = results.join(";");

        if !results_string.ends_with(";") && results_string.len() > 0 {
            results_string = results_string + ";";
        }

        if results_string.len() == 0 {
            results_string = " ".to_string();
        }

        let _ = self.socket.write_all(results_string.as_bytes()).await;
    }

    async fn handle_client(mut self) {
        let mut buf = [0; 1024];
        let mut session = Session::new();

        loop {
            match self.socket.read(&mut buf).await {
                Ok(0) => return,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
                Ok(n) => {
                    let line = self.parse_line(buf, n);

                    let commands = self.split_line(line);

                    let mut commands = self.continue_command(&mut session, commands);

                    if !commands[commands.len() - 1].ends_with(";") {
                        session.incomplete_command = commands[commands.len() - 1].to_string();
                        commands.pop();
                    }

                    let commands = commands
                        .iter()
                        .map(|line| line.trim_end_matches(";"))
                        .collect::<Vec<&str>>();

                    let mut results = Vec::new();

                    for line in commands {
                        match Command::from_str(line) {
                            Ok(cmd) => {
                                results.push(
                                    self.handle_command_result(
                                        self.execute_command(
                                            Arc::clone(&self.data),
                                            session.clone(),
                                            cmd,
                                        )
                                        .await,
                                        &mut session,
                                    ),
                                );
                            }
                            Err(e) => {
                                results.push(e.to_string());
                            }
                        }
                    }

                    self.write_results(results).await;
                }
            }
        }
    }

    pub async fn spawn_handler(self) {
        tokio::spawn(async move {
            self.handle_client().await;
        });
    }
}
