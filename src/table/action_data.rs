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

use crate::table::ToBytes;

/// Represents data associated with an action.
///
/// The data represents a single value, e.g., `string` or `bool`.
/// For further information, see [Open-Tofino protobuf definition](https://github.com/barefootnetworks/Open-Tofino/blob/master/share/bf_rt_shared/proto/bfruntime.proto).
#[derive(Debug, Clone)]
pub struct ActionData {
    /// Name of the action parameter
    key: String,
    /// Data of the action parameter.
    /// The value is converted to a byte vector.
    data: Vec<u8>,
}

/// Represents data associated with an action.
///
/// The data represents a list of values, e.g., `IntArray` or `BoolArray`.
/// For further information, see [Open-Tofino protobuf definition](https://github.com/barefootnetworks/Open-Tofino/blob/master/share/bf_rt_shared/proto/bfruntime.proto).
#[derive(Debug, Clone)]
pub struct ActionDataRepeated {
    /// Name of the action parameter.
    key: String,
    /// List of data of the action parameter.
    /// Each value is converted to a byte vector.
    data: Vec<Vec<u8>>,
}

impl ActionData {
    /// Creates a new [ActionData] mapping the `key` to the `data`.
    pub fn new<T: ToBytes>(key: &str, data: T) -> Self {
        ActionData {
            key: key.to_owned(),
            data: data.to_bytes(),
        }
    }

    /// Returns the `key` mapped to the `data`.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Returns the `data` associated with the `key`.
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Returns the data as an [u32].
    pub fn as_u32(&self) -> u32 {
        self.get_data().to_u32()
    }

    /// Returns the data as an [u64].
    pub fn as_u64(&self) -> u64 {
        self.get_data().to_u64()
    }    

    /// Returns the data as an [u128].
    pub fn as_u128(&self) -> u128 {
        self.get_data().to_u128()
    }
    
}

impl ActionDataRepeated {
    /// Creates a new [ActionDataRepeated] mapping the `key` to the `data`.
    pub fn new<T: ToBytes>(key: &str, data: Vec<T>) -> Self {
        ActionDataRepeated {
            key: key.to_owned(),
            data: data.into_iter().map(|d| d.to_bytes()).collect(),
        }
    }

    /// Returns the `key` mapped to the `data`.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    /// Returns the `data` associated with the `key`.
    pub fn get_data(&self) -> &Vec<Vec<u8>> {
        &self.data
    }
}
