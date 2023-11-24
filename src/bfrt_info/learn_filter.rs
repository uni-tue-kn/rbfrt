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
use crate::error::RBFRTError;
use crate::error::RBFRTError::UnknownLearnFilterField;

#[derive(Deserialize, Debug, Clone)]
pub struct LearnFilterObject {
    pub name: String,
    pub id: u32,
    pub fields: Vec<LearnFilterField>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LearnFilterField {
    name: String,
    id: u32,
}

impl LearnFilterObject {
    pub fn get_data_field_name_by_id(&self, id: u32) -> Result<String, RBFRTError> {
        for field in &self.fields {
            if field.id == id {
                return Ok(field.name.to_owned());
            }
        }

        Err(UnknownLearnFilterField {field_id: id})

    }
}

