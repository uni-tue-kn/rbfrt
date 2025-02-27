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
#[derive(Deserialize, Debug, Clone)]
pub enum TableMatchTypes {
    LPM,
    Exact,
    Range,
    Ternary,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BFRTFieldType {
    pub(crate) r#type: String,
    pub(crate) width: Option<u32>,
}

impl BFRTFieldType {
    pub fn get_width(&self) -> u32 {
        match self.r#type.as_str() {
            "uint64" => 64,
            "uint32" => 32,
            "uint16" => 16,
            "uint8" => 8,
            "bytes" => self.width.unwrap(),
            "bool" => 1,
            "string" => 32, // strings are handled seperately
            _ => panic!("Unknown width type: {}", self.r#type.as_str()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum TableType {
    #[serde(alias = "MatchAction_Direct")]
    MatchActionDirect,
    Register,
    Meter,
    SnapshotCfg,
    SnapshotTrigger,
    SnapshotData,
    SnapshotLiveness,
    PortMetadata,
    PreMgid,
    PreNode,
    PreEcmp,
    PreLag,
    PrePrune,
    PrePort,
    PortConfigure,
    PortStat,
    PortHdlInfo,
    PktgenAppCfg,
    PktgenPktBufferCfg,
    #[serde(other)]
    Unknown,
}
