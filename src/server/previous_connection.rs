use std::sync::mpsc::Sender;

use async_std::task;
use lib::{
    common_errors::ConnectionError,
    connection_protocol::ConnectionProtocol,
    serializer::deserialize,
};

use crate::server_messages::ServerMessage;

pub struct PrevConnection {
    connection: Box<dyn ConnectionProtocol + Send>,
    to_next_sender: Sender<ServerMessage>,
    to_orders_manager_sender: Sender<ServerMessage>,
}

impl PrevConnection {
    pub fn new(
        connection: Box<dyn ConnectionProtocol + Send>,
        to_next_sender: Sender<ServerMessage>,
        to_orders_manager_sender: Sender<ServerMessage>
    ) -> PrevConnection {
        PrevConnection { connection, to_next_sender, to_orders_manager_sender }
    }

    pub fn listen(&mut self) -> Result<(), ConnectionError> {
        loop {
            let encoded = task::block_on(self.connection.recv());
            if encoded.is_err() {
                // TODO handle lost connection
                return Err(ConnectionError::ConnectionLost);
            }
            let mut encoded = encoded.unwrap();
            let message: ServerMessage = deserialize(&mut encoded)?;

            // TODO handle message
            // si es token --> agrego los cambios que vengan y saco los mios viejos, despierto al hilo, paso el token para que lo agarre next
            // si es hello --> pasar a next
            // si es close --> terminamos la conexion
        }
    }
}