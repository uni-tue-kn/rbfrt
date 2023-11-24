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

/// Represents a match value.
///
/// Example:
/// ```
/// use rbfrt::table::MatchValue;
/// MatchValue::lpm(vec![10, 0, 0, 2], 32);
/// ```

use crate::table::ToBytes;

#[derive(Debug, Clone)]
pub enum MatchValue {
    ExactValue { bytes: Vec<u8> },
    RangeValue { lower_bytes: Vec<u8>, higher_bytes: Vec<u8> },
    LPM { bytes: Vec<u8>, prefix_length: i32 },
    Ternary { value: Vec<u8>, mask: Vec<u8> }
}


impl MatchValue {
    /// Creates a new Exact match value
    /// ```
    /// use rbfrt::table::MatchValue;
    /// MatchValue::exact(10);
    /// ```
    pub fn exact<T: ToBytes>(value: T) -> MatchValue {
        MatchValue::ExactValue { bytes: value.to_bytes() }
    }

    pub fn get_exact_value(&self) -> &Vec<u8> {
        match self {
            MatchValue::ExactValue { bytes } => bytes,
            _ => panic!("No exact match value.")
        }
    }

    /// Creates a new Range match value
    /// ```
    /// use rbfrt::table::MatchValue;
    /// MatchValue::range(20, 30);
    /// ```
    pub fn range<T: ToBytes>(lower: T, higher: T) -> MatchValue {
        MatchValue::RangeValue {lower_bytes: lower.to_bytes(), higher_bytes: higher.to_bytes()}
    }

    pub fn get_range_value(&self) -> (&Vec<u8>, &Vec<u8>) {
        match self {
            MatchValue::RangeValue {lower_bytes, higher_bytes } => (lower_bytes, higher_bytes),
            _ => panic!("No range match value.")
        }
    }

    /// Creates a new LPM match value
    /// ```
    /// use rbfrt::table::MatchValue;
    /// MatchValue::lpm(vec![10, 0, 0, 2], 32);
    /// ```
    pub fn lpm<T: ToBytes>(value: T, prefix_length: i32) -> MatchValue {
        MatchValue::LPM {bytes: value.to_bytes(), prefix_length}
    }

    /// Creates a new Ternary match value
    /// ```
    /// use rbfrt::table::MatchValue;
    /// MatchValue::ternary(vec![7, 0, 7], vec![0, 0, 1]);
    /// ```
    pub fn ternary<T: ToBytes>(value: T, mask: T) -> MatchValue {
        MatchValue::Ternary {value: value.to_bytes(), mask: mask.to_bytes()}
    }
}
