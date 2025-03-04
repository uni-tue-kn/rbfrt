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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a digest/message sent from the switch to the controller.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Digest {
    /// Name of the [Digest] instance used in the P4 program.
    pub name: String,
    /// The data contained in the [Digest].
    /// It contains the mapping of the key and its value send with the `pack` method from the P4 program.
    ///
    /// Suppose the EtherType of an Ethernet header is included in the [Digest].
    /// Then, the key contains the identifier used by the P4 program, e.g., `ether_type`, and the value contains its `bit<16>` value
    pub data: HashMap<String, Vec<u8>>,
}
