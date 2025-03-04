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

use crate::error::RBFRTError;
use crate::table::action_data::ActionDataRepeated;
use crate::table::{ActionData, MatchValue, ToBytes};
use std::collections::HashMap;

/// Represents a table entry internally.
#[derive(Debug)]
pub struct TableEntry {
    /// Id of the table.
    pub table_id: u32,
    /// Name of the table.
    pub table_name: String,
    /// Names and their values of the match keys.
    pub match_keys: HashMap<String, MatchValue>,
    /// Flag indicating if this is the default entry for the table.
    pub default_entry: bool,
    /// Name of the associated action
    pub action: String,
    /// Action data of the action, empty if not parameters are provided.
    pub action_data: Vec<ActionData>,
}

impl TableEntry {
    /// Returns the [MatchValue] of the match key with the given `name`.
    pub fn get_key(&self, name: &str) -> Result<&MatchValue, RBFRTError> {
        if self.match_keys.contains_key(name) {
            Ok(self.match_keys.get(name).unwrap())
        } else {
            Err(RBFRTError::UnknownKeyName {
                name: name.to_string(),
                table_name: self.table_name.clone(),
            })
        }
    }

    /// Returns whether a match key with the given `name` is present.
    pub fn has_key(&self, name: &str) -> bool {
        self.match_keys.contains_key(name)
    }

    /// Returns the [ActionData] which key has the given `name`.
    pub fn get_action_data(&self, name: &str) -> Result<&ActionData, RBFRTError> {
        for action in &self.action_data {
            if action.get_name() == name {
                return Ok(action);
            }
        }

        Err(RBFRTError::UnknownActionName {
            name: name.to_string(),
        })
    }

    /// Returns whether an action data has a key with the given `name`.
    pub fn has_action_data(&self, name: &str) -> bool {
        self.get_action_data(name).is_ok()
    }

    /// Returns the action's name.
    pub fn get_action_name(&self) -> &str {
        &self.action
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RequestType {
    Read,
    Write,
    Update,
    Operation,
    Delete,
}

/// Represents all possible table operations.
#[derive(Debug, Clone)]
pub enum TableOperation {
    None,
    SyncCounters,
    SyncRegister,
}

impl TableOperation {
    pub fn get_string(&self) -> String {
        match self {
            TableOperation::None => String::from(""),
            TableOperation::SyncCounters => String::from("SyncCounters"),
            TableOperation::SyncRegister => String::from("SyncRegisters"),
        }
    }
}

/// Represents a table entry request.
///
/// # Example
///
/// ```
/// use rbfrt::table::{Request, MatchValue};
/// Request::new("ingress.p4tg.frame_type.frame_type_monitor")
///      .match_key("hdr.ipv4.dst_addr", MatchValue::lpm(vec![10u8, 0, 0, 2], 32))
///      .match_key("ig_intr_md.ingress_port", MatchValue::exact(0));
/// ```
#[derive(Debug, Clone)]
pub struct Request {
    /// Name of the table.
    pub table_name: String,
    /// List of match keys.
    match_keys: HashMap<String, MatchValue>,
    /// Name of the action.
    ///
    /// # Note
    ///
    /// Only required for write / update requests
    action: Option<String>,
    /// Associated data of the action.
    action_data: Vec<ActionData>,
    action_data_repeated: Vec<ActionDataRepeated>,
    request_type: RequestType,
    operation: TableOperation,
    is_default_entry: bool,
    pipe: Option<u32>,
}

impl Request {
    pub fn new(table_name: &str) -> Request {
        Request {
            table_name: table_name.to_owned(),
            match_keys: HashMap::new(),
            action: None,
            action_data: vec![],
            action_data_repeated: vec![],
            request_type: RequestType::Read,
            operation: TableOperation::None,
            is_default_entry: false,
            pipe: None,
        }
    }

    /// sets the list of the requested match keys
    pub fn match_keys(mut self, match_keys: HashMap<String, MatchValue>) -> Request {
        self.match_keys = match_keys;
        self
    }

    /// sets a match key that is requested
    pub fn match_key(mut self, name: &str, match_value: MatchValue) -> Request {
        self.match_keys.insert(name.to_owned(), match_value);
        self
    }

    pub fn action(mut self, action: &str) -> Request {
        self.action = Some(action.to_owned());
        self
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    pub fn action_data<T: ToBytes>(mut self, name: &str, data: T) -> Request {
        self.action_data.push(ActionData::new(name, data));
        self
    }

    pub fn default(mut self, is_default: bool) -> Request {
        self.is_default_entry = is_default;
        self
    }

    pub fn pipe(mut self, pipe: u32) -> Request {
        self.pipe = Some(pipe);
        self
    }

    pub fn get_pipe(&self) -> Option<u32> {
        self.pipe
    }

    pub fn is_default(&self) -> bool {
        self.is_default_entry
    }

    pub fn action_data_repeated<T: ToBytes>(mut self, name: &str, data: Vec<T>) -> Request {
        self.action_data_repeated
            .push(ActionDataRepeated::new(name, data));
        self
    }

    pub(crate) fn request_type(mut self, request_type: RequestType) -> Request {
        self.request_type = request_type;
        self
    }

    pub fn operation(mut self, operation: TableOperation) -> Request {
        self.operation = operation;
        self
    }

    pub fn get_action_name(&self) -> &str {
        self.action.as_ref().unwrap()
    }

    pub fn get_action_data(&self) -> &Vec<ActionData> {
        &self.action_data
    }

    pub fn get_table_name(&self) -> &str {
        &self.table_name
    }

    pub fn get_action_data_repeated(&self) -> &Vec<ActionDataRepeated> {
        &self.action_data_repeated
    }
    /// returns match keys
    pub fn get_match_keys(&self) -> &HashMap<String, MatchValue> {
        &self.match_keys
    }

    pub(crate) fn get_type(&self) -> &RequestType {
        &self.request_type
    }

    pub(crate) fn get_operation(&self) -> &TableOperation {
        &self.operation
    }
}
