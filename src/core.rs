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
#[derive(Deserialize, Debug)]
pub(crate) struct Configuration {
    pub(crate) p4_devices: Vec<DeviceConfig>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DeviceConfig {
    #[serde(alias = "device-id")]
    #[allow(dead_code)]
    pub(crate) device_id: u32,
    pub(crate) p4_programs: Vec<P4Program>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct P4Program {
    #[serde(alias = "program-name")]
    pub(crate) program_name: String,
    #[serde(alias = "bfrt-config")]
    pub(crate) bfrt_config: String,
    pub(crate) p4_pipelines: Vec<P4Pipeline>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct P4Pipeline {
    pub(crate) p4_pipeline_name: String,
    pub(crate) context: String,
    pub(crate) config: String,
    pub(crate) pipe_scope: Vec<u32>,
}
