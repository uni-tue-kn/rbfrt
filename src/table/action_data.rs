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

/// Represents the data of an action
#[derive(Debug, Clone)]
pub struct ActionData {
    /// name of the action parameter
    key: String,
    /// data of the action parameter
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ActionDataRepeated {
    /// name of the action parameter
    key: String,
    /// data of the action parameter
    data: Vec<Vec<u8>>,
}

impl ActionData {
    pub fn new<T: ToBytes>(key: &str, data: T) -> Self {
        ActionData {
            key: key.to_owned(),
            data: data.to_bytes(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.key
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn as_u32(&self) -> u32 {
        self.get_data().to_u32()
    }
}

impl ActionDataRepeated {
    pub fn new<T: ToBytes>(key: &str, data: Vec<T>) -> Self {
        ActionDataRepeated {
            key: key.to_owned(),
            data: data.into_iter().map(|d| d.to_bytes()).collect(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.key
    }

    pub fn get_data(&self) -> &Vec<Vec<u8>> {
        &self.data
    }
}
