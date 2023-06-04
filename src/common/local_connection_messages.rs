use serde::{Deserialize, Serialize};

use crate::common_errors::ConnectionError;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CoffeeMakerRequest {
    pub message_type: MessageType,
    pub account_id: usize,
    pub points: usize,
}

#[derive(Deserialize, Serialize)]
pub struct CoffeeMakerResponse {
    pub message_type: MessageType,
    pub status: ResponseStatus,
}

#[derive(Deserialize, Serialize)]
pub enum ResponseStatus {
    Ok,
    Err(ConnectionError),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MessageType {
    AddPoints,
    RequestPoints,
    TakePoints,
    CancelPointsRequest,
}