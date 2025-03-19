/* Copyright 2023-present University of Tuebingen, Chair of Communication Networks
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * Steffen Lindner (steffen.lindner@uni-tuebingen.de)
 */

use std::error::Error;

use crate::error::RBFRTError::GRPCError;
use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum RBFRTError {
    #[error("Connection to `{ip}:{port}` not possible. Original: `{orig_e}`")]
    ConnectionError {
        ip: String,
        port: u16,
        orig_e: Box<dyn Error>,
    },
    #[error("Unable to get forwarding pipeline for device_id {device_id} and client_id {client_id}. Original: `{orig_e}`")]
    GetForwardingPipelineError {
        device_id: u32,
        client_id: u32,
        orig_e: Box<dyn Error>,
    },
    #[error("P4 program {name} does not exist.")]
    P4ProgramError { name: String },
    #[error("Pipe {pipe_id} does not exist.")]
    PipeError { pipe_id: u8 },
    #[error("Action id {action_id} does not exist.")]
    UnknownActionId { action_id: u32 },
    #[error("Action {name} does not exist.")]
    UnknownActionName { name: String },
    #[error("Singleton/Register param {name} does not exist.")]
    UnknownSingletonName { name: String },
    #[error("Register/Singleton param id {id} does not exist.")]
    UnknownSingletonId { id: u32 },
    #[error("Table {table_name} does not have key with id {id}.")]
    UnknownKeyId { id: u32, table_name: String },
    #[error("Table {table_name} does not have key {name}.")]
    UnknownKeyName { name: String, table_name: String },
    #[error("Table {table_name} does not exist.")]
    UnknownTable { table_name: String },
    #[error("Table id {table_id} does not exist.")]
    UnknownTableId { table_id: u32 },
    #[error("Read result was not a table entry.")]
    UnknownReadResult {},
    #[error("Learn filter with id {filter_id} does not exist.")]
    UnknownLearnFilter { filter_id: u32 },
    #[error("Learn filter field with id {field_id} does not exist.")]
    UnknownLearnFilterField { field_id: u32 },
    #[error("Value {value:?} does not fit into {name} with width {width} bits.")]
    ConvertError {
        value: Vec<u8>,
        name: String,
        width: u32,
    },
    #[error("Action data with id {id} does not exist on action {action_name}.")]
    UnknownActionDataId { id: u32, action_name: String },
    #[error("Action data {name} does not exist on action {action_name}.")]
    UnknownActionDataName { name: String, action_name: String },
    #[error("Port {name} does not exist.")]
    PortNotFound { name: String },
    #[error("GRPC error: {message}. Details: {details}.")]
    GRPCError { message: String, details: String },
    #[error("Register index is missing.")]
    MissingRegisterIndex,
    #[error("Cannot convert Bytes to {target}. Original: `{orig_e}`")]
    ByteConversionError {
        target: String,
        orig_e: Box<dyn Error>,
    },
    #[error("Switch request is empty.")]
    RequestEmpty {},
    #[error("Generic error occurred. Message: {message}.")]
    GenericError { message: String },
}

impl From<Status> for RBFRTError {
    fn from(value: Status) -> Self {
        GRPCError {
            message: value.message().to_owned(),
            details: format!("{:#?}", value),
        }
    }
}

unsafe impl Send for RBFRTError {}
unsafe impl Sync for RBFRTError {}
