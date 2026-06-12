use std::{collections::{HashMap, VecDeque}, fs::read_to_string, sync::mpsc::{self, Receiver, Sender}, thread::{JoinHandle, spawn}};
use message_io::{network::{Endpoint, NetEvent, Transport}, node::{self, NodeHandler, NodeTask}};
use shared::{message::{AccountCreationResult, AccountLoginResult, ClientToServerMessage, ServerToClientMessage}, warn};

use crate::network::logic::{ServerData, does_user_exist, login_user, register_user};

#[derive(Clone, Copy)]
pub struct AccountRequirements {
    pub password_min_length: usize,
    pub password_max_length: usize,
    pub username_min_length: usize,
    pub username_max_length: usize
}

#[derive(Clone, Copy)]
pub struct ServerConfig {
    pub port: u32,
    pub account_requirements: AccountRequirements
}

pub struct Client {
    username: String
}

pub struct NetworkController {
    config: ServerConfig,
    server_data: ServerData,
    task: NodeTask,
    clients: HashMap<Endpoint, Client>,
    message_queue: Receiver<(Endpoint, ClientToServerMessage)>,
    handler: NodeHandler<()>
}

impl NetworkController {

    /**
     * second element is true if the data already exists
     */
    pub fn load_server_data() -> (ServerData, bool) {
        let content = read_to_string("res/server/serverdata.json");

        if let Ok(c) = content {
            let data: Result<ServerData, _> = serde_json::from_str(&c);
            if let Ok(d) = data {
                (d, true)
            }
            else {
                (ServerData::default(), false)
            }
        }
        else {
            (ServerData::default(), false)
        }
    }

    pub async fn new() -> Self {

        let config = ServerConfig {
            port: 13654,
            account_requirements: AccountRequirements {
                password_max_length: 52,
                password_min_length: 5,
                username_min_length: 3,
                username_max_length: 14
            }
        };

        let (server_data, loaded_server_data_from_disk) = NetworkController::load_server_data();

        if !loaded_server_data_from_disk {
            warn!("Server Data was loaded from disk");
        }

        let (sender, recv) = mpsc::channel(); 

        let (handler, listener) = node::split::<()>();
        
        handler.network().listen(Transport::FramedTcp, format!("0.0.0.0:{}", config.port)).unwrap();
        handler.network().listen(Transport::Udp, format!("0.0.0.0:{}", config.port)).unwrap();

        let task = listener.for_each_async(move |event| match event.network() {
            NetEvent::Connected(_, _) => unreachable!(),
            NetEvent::Accepted(endpoint, listener) => {
                
            },
            NetEvent::Message(endpoint, data) => {
                if let Ok(message) = bincode::deserialize::<ClientToServerMessage>(data) {
                    sender.send((endpoint, message)).unwrap();
                }
                else {
                    warn!("Client sent bad message");
                }
            },
            NetEvent::Disconnected(endpoint) => {

            }
        });

        Self {
            config: config.clone(),
            server_data,
            task,
            clients: HashMap::new(),
            message_queue: recv,
            handler
        }
    }
    pub fn update(&mut self) {
        while let Ok((endpoint, message)) = self.message_queue.try_recv() {
            match message {
                ClientToServerMessage::ConnectionStart(username) => {
                    let data = if does_user_exist(&self.server_data, &username) {
                        ServerToClientMessage::ConnectionRequestLogin
                    }
                    else {
                        ServerToClientMessage::ConnectionRequestRegister
                    };
                    let ser = bincode::serialize(&data).unwrap();
                    self.handler.network().send(endpoint, &ser);
                },
                ClientToServerMessage::RegisterAccount { username, password } => {
                    let data = if does_user_exist(&self.server_data, &username) {
                        ServerToClientMessage::AccountCreationResult(AccountCreationResult::UsernameAlreadyExists)
                    }
                    else {
                        ServerToClientMessage::AccountCreationResult(register_user(&mut self.server_data, &self.config, username, password))
                    };
                    let ser = bincode::serialize(&data).unwrap();
                    self.handler.network().send(endpoint, &ser);
                },
                ClientToServerMessage::LoginAccount { username, password } => {
                    let data = login_user(&self.server_data, username, password);

                    if data == AccountLoginResult::OK {
                        //login the user
                    }
                    let ser = bincode::serialize(&ServerToClientMessage::AccountLoginResult(data)).unwrap();
                    self.handler.network().send(endpoint, &ser);
                }
            }
        }
    }
}