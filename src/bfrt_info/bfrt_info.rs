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

use serde::Deserialize;

use crate::bfrt_info::{BFRTTableObject, LearnFilterObject};
use crate::error::RBFRTError;
use crate::error::RBFRTError::{ConvertError, UnknownLearnFilter, UnknownTable, UnknownTableId};

/// a struct into which to decode the thing
#[derive(Deserialize, Debug, Clone)]
pub struct BFRTInfo {
    tables: Vec<BFRTTableObject>,
    learn_filters: Option<Vec<LearnFilterObject>>
}

impl BFRTInfo {
    pub fn table_get(&self, name: &str) -> Result<&BFRTTableObject, RBFRTError> {
        for t in &self.tables {
            if t.name() == format!("pipe.{}", name) || t.name() == name {
                return Ok(t);
            }
        }

        Err(UnknownTable { table_name: name.to_owned()})
    }

    pub fn table_get_by_id(&self, id: u32) -> Result<&BFRTTableObject, RBFRTError> {
        for t in &self.tables {
            if t.id() == id {
                return Ok(t);
            }
        }

        Err(UnknownTableId { table_id: id})
    }


    pub fn tables(self) -> Vec<BFRTTableObject> {
        self.tables
    }

    pub fn add_table(&mut self, table: BFRTTableObject) {
        self.tables.push(table);
    }

    pub fn learn_filter_get(&self, id: u32) -> Result<&LearnFilterObject, RBFRTError> {
        if self.learn_filters.is_none() {
            return Err(UnknownLearnFilter {filter_id: id});
        }

        for l in self.learn_filters.as_ref().unwrap() {
            if l.id == id {
                return Ok(l);
            }
        }

        Err(UnknownLearnFilter {filter_id: id})

    }
}

pub(crate) trait Convert {
    /// Converts a data type into a byte vector with <width> bits
    ///
    /// * `name` - Name of the field; used for error message
    /// * `width` - Width in bits
    fn convert(self, name: &str, width: u32) -> Result<Vec<u8>, RBFRTError>;
}

impl Convert for Vec<u8> {
    fn convert(mut self, name: &str, width: u32) -> Result<Vec<u8>, RBFRTError> {
        // hack for string type
        if width == u32::MAX {
            return Ok(self);
        }

        // remove leading zeros
        for e in self.clone() {
            if e == 0u8 {
                self.remove(0);
            } else {
                break;
            }
        }

        let num_bytes = (width as f32) / 8f32;
        let num_bytes = num_bytes.ceil() as u32;

        if num_bytes < self.len() as u32 {
            println!("num bytes {} self.len {}", num_bytes, self.len());
            return Err(ConvertError {
                value: self.clone(),
                name: name.to_owned(),
                width,
            });
        }

        let diff = num_bytes - self.len() as u32;

        let mut ret: Vec<u8> = vec![0; diff as usize];

        ret.append(&mut self);

        Ok(ret)
    }
}







