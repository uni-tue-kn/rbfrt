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

use crate::bfrt::BFRTFieldType;
use crate::error::RBFRTError;
use crate::error::RBFRTError::{UnknownActionDataId, UnknownActionDataName};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
enum ActionScope {
    TableAndDefault,
    DefaultOnly,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct ActionData {
    #[serde(default = "default_id")]
    id: u32,
    #[serde(default = "default_name")]
    name: String,
    repeated: Option<bool>,
    mandatory: Option<bool>,
    read_only: Option<bool>,
    r#type: Option<BFRTFieldType>,
}

fn default_id() -> u32 {
    0
}

fn default_name() -> String {
    "Unknown name".to_string()
}

impl ActionData {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BFRTAction {
    pub(crate) id: u32,
    pub(crate) name: String,
    #[allow(dead_code)]
    action_scope: Option<ActionScope>,
    data: Option<Vec<ActionData>>,
}

impl BFRTAction {
    pub fn get_action_data_by_id(&self, id: u32) -> Result<&ActionData, RBFRTError> {
        if let Some(data) = &self.data {
            for d in data {
                if d.id == id {
                    return Ok(d);
                }
            }
        }

        Err(UnknownActionDataId {
            id,
            action_name: self.name.clone(),
        })
    }

    pub fn get_action_data_by_name(&self, name: &str) -> Result<&ActionData, RBFRTError> {
        if let Some(data) = &self.data {
            for d in data {
                if d.name == *name {
                    return Ok(d);
                }
            }
        }

        Err(UnknownActionDataName {
            name: name.to_string(),
            action_name: self.name.clone(),
        })
    }

    pub fn get_action_data_type(&self, name: &str) -> Result<&BFRTFieldType, RBFRTError> {
        let action_data = self.get_action_data_by_name(name)?;

        Ok(action_data.r#type.as_ref().unwrap())
    }
}
