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
use crate::bfrt_info::{BFRTFieldType, TableMatchTypes};

#[derive(Deserialize, Debug, Clone)]
pub struct BFRTTableKeyObject {
    id: u32,
    name: String,
    #[allow(dead_code)]
    repeated: Option<bool>,
    #[allow(dead_code)]
    mandatory: bool,
    #[allow(dead_code)]
    match_type: TableMatchTypes,
    r#type: BFRTFieldType,
}

impl BFRTTableKeyObject {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> &BFRTFieldType {
        &self.r#type
    }

}