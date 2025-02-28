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

use crate::table;
use crate::table::{TableEntry, ToBytes};
use std::collections::HashMap;

/// register index type
/// default 32-bit
pub type IndexType = u32;

/// Represents a register that may contain values at several indices.
#[derive(Debug, Clone)]
pub struct Register {
    /// name of the register
    name: String,
    /// entires of the register
    entries: HashMap<IndexType, RegisterEntry>,
}

impl Register {
    pub fn new(name: &str, entries: HashMap<IndexType, RegisterEntry>) -> Register {
        Register {
            name: name.to_owned(),
            entries,
        }
    }

    /// Returns the name of the register
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all entries of the register
    pub fn entries(&self) -> &HashMap<IndexType, RegisterEntry> {
        &self.entries
    }

    /// Returns an entry at a specific index of the register
    pub fn get(&self, index: IndexType) -> Option<&RegisterEntry> {
        self.entries.get(&index)
    }

    pub fn parse_register_entries(entries: Vec<TableEntry>, name: &str) -> Register {
        // convert regular table entry to register entry
        let mut register_entries: HashMap<IndexType, RegisterEntry> = HashMap::new();

        for e in entries {
            let index = table::ToBytes::to_u32(
                e.match_key
                    .get("$REGISTER_INDEX")
                    .unwrap()
                    .get_exact_value(),
            );
            let mut reg_data: HashMap<String, Vec<Vec<u8>>> = HashMap::new();

            for data in e.action_data {
                let key = data.get_name();
                let value = data.get_data();

                // key already exists
                // value for different pipe
                if reg_data.contains_key(key) {
                    reg_data.get_mut(key).unwrap().push(value.clone());
                } else {
                    reg_data.insert(key.to_owned(), vec![value.clone()]);
                }
            }

            register_entries.insert(index, RegisterEntry::new(index, reg_data));
        }

        Register::new(name, register_entries)
    }
}

/// Represents a register entry at a specific index
#[derive(Debug, Clone)]
pub struct RegisterEntry {
    index: IndexType,
    data: HashMap<String, Vec<Vec<u8>>>,
}

impl RegisterEntry {
    pub(crate) fn new(index: IndexType, data: HashMap<String, Vec<Vec<u8>>>) -> RegisterEntry {
        RegisterEntry { index, data }
    }

    /// Returns the index of the register entry
    pub fn get_index(&self) -> IndexType {
        self.index
    }

    /// Returns all data of the register entry.
    /// Each data entry contains a `Vec<u8>` for each pipe
    pub fn get_data(&self) -> &HashMap<String, Vec<Vec<u8>>> {
        &self.data
    }

    /// Returns a specific data field of the entry.
    /// Each data entry contains a `Vec<u8>` for each pipe
    ///
    /// * `name` - Name of the data field
    pub fn get(&self, name: &str) -> Option<&Vec<Vec<u8>>> {
        self.data.get(name)
    }
}

/// Represents a register request
#[derive(Debug, Clone)]
pub struct Request {
    name: String,
    index: Option<IndexType>,
    data: HashMap<String, Vec<u8>>,
}

impl Request {
    pub fn new(name: &str) -> Request {
        Request {
            name: name.to_owned(),
            index: None,
            data: HashMap::new(),
        }
    }

    pub fn index(self, index: IndexType) -> Request {
        Request {
            index: Some(index),
            ..self
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_index(&self) -> &Option<IndexType> {
        &self.index
    }

    pub fn data<T: ToBytes>(mut self, name: &str, value: T) -> Request {
        self.data.insert(name.to_owned(), value.to_bytes());
        self
    }

    pub fn get_data(&self) -> &HashMap<String, Vec<u8>> {
        &self.data
    }
}
