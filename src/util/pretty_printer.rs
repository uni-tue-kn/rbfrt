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
 * Fabian Ihle (fabian.ihle@uni-tuebingen.de)
 */

use crate::error::RBFRTError;
use crate::table::{MatchValue, TableEntry, ToBytes};
use std::collections::BTreeSet;
use std::collections::HashMap;

use prettytable::{format, Row, Table};

pub struct PrettyPrinter {
    infer_address_type_flag: bool,
}

/// PrettyPrinter to display MATs and their entries.
impl PrettyPrinter {

    pub fn new() -> PrettyPrinter {
        PrettyPrinter {
            infer_address_type_flag: true,
        }
    }

    /// Set the infer_address_type_flag
    /// * `infer_address_type_flag` - The PrettyPrinter will try to infer MAC and IP addresses from column names
    pub fn infer_address_type_flag(self, infer_address_type_flag: bool) -> PrettyPrinter {
        PrettyPrinter {infer_address_type_flag}
    }

    pub fn get_infer_address_type_flag(&self) -> bool {
        self.infer_address_type_flag
    }

    /// Converts data in the form of &Vec<u8> into strings
    /// Up to a length of 128 bytes, the bytearray is converted to a number
    /// If the get_infer_address_type_flag is set, this function will try to infer
    /// the address type (MAC and IPv4) from the column name and data length
    ///
    /// # Attributes
    /// * `key` - The column name for this data
    /// * `data` - The data to be converted
    fn convert_data_to_string(&self, key: &str, data: &Vec<u8>) -> String {
        let mut address: String;

        if data.len() <= 4 {
            address = ToString::to_string(&data.to_u32());
        }
        else if data.len() <= 8 {
            address = data.to_u64().to_string();
        }
        else if data.len() <= 16 {
            address = data.to_u128().to_string();
        } 
        else {
            address = format!("{:?}", data);
        }

        if self.get_infer_address_type_flag() {
            if key.contains("addr") || key.contains("address") {
                if data.len() == 6 {
                    // possibly a mac address
                    address = format!(
                        "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                        data[0], data[1], data[2], data[3], data[4], data[5]
                    );
                } else if data.len() == 4 {
                    // possible an IPv4 adress
                    address = format!("{}.{}.{}.{}", data[0], data[1], data[2], data[3]);
                }
            }
        }

        address
    }

    /// Create the header row entry for this table, consisting of all match keys, and the action name and parameter column
    ///
    /// # Attributes
    /// * `entries` - All entries that will be in this table
    fn create_header_row(&self, entries: &[TableEntry]) -> BTreeSet<String> {
        let mut header_row: BTreeSet<String> = BTreeSet::new();
        header_row.insert("Action".to_string());
        header_row.insert("Action parameters".to_string());

        // Collect all possible key columns first before processing data
        if !entries.is_empty() {
            let entry = &entries[0];
            let mut col_name: String;
            for key in &entry.match_key {
                match key.1 {
                    MatchValue::ExactValue { bytes: _} => {
                        col_name = format!("EXT:{}", key.0);
                    }
                    MatchValue::LPM { bytes: _, prefix_length: _} => {
                        col_name = format!("LPM:{}", key.0);
                    }
                    MatchValue::RangeValue { lower_bytes: _, higher_bytes: _} => {
                        col_name = format!("RNG:{}", key.0);
                    }
                    MatchValue::Ternary { value: _, mask : _} => {
                        col_name = format!("TER:{}", key.0);
                    }
                }
                
                header_row.insert(col_name);
            }
        }

        header_row
    }


    /// Creates a data row for a table. Requires the header row to be build first.
    ///
    /// # Attributes
    /// * `entry` - The entry to be added to the table
    /// * `header_row` - The header row set built with `create_header_row`
    fn create_data_row(&self, entry: &TableEntry, header_row: &BTreeSet<String>) -> Vec<String> {
        let mut row_entry: Vec<String> = Vec::new();

        // The BTreeSet is ordered alphabetically.
        // Therefore, iterate all data fields, check for every possible column,
        // and insert the found values in the correct order
        for col in header_row {
            if *col == "Action" {
                // Write action name into the col
                let action_name = &entry.action;
                row_entry.push(action_name.clone());
            }
            else if *col == "Action parameters" {
                let action_data_table = self.create_action_sub_table(entry);
                row_entry.push(action_data_table);
            }
            else {
                // Collect data for keys in MAT
                let key_name = col.get(4..);
                match key_name {
                    Some(key) => {
                        let values: Vec<&MatchValue> = entry
                            .match_key
                            .iter()
                            .filter(|entry| entry.0 == key)
                            .map(|entry| entry.1)
                            .collect();
                        if !values.is_empty() {
                            let value = &values[0];
                            match value {
                                MatchValue::ExactValue { bytes } => {
                                    let bytes_str = self.convert_data_to_string(key, bytes);
                                    row_entry.push(bytes_str)
                                }
                                MatchValue::LPM {
                                    bytes,
                                    prefix_length,
                                } => {
                                    let bytes_str = self.convert_data_to_string(key, bytes);
                                    let lpm_str = format!("{bytes_str} / {prefix_length}");
                                    row_entry.push(lpm_str);
                                }
                                MatchValue::RangeValue {
                                    lower_bytes,
                                    higher_bytes,
                                } => {
                                    let lower_str = self.convert_data_to_string(key, lower_bytes);
                                    let upper_str = self.convert_data_to_string(key, higher_bytes);
                                    let range_str = format!("[{lower_str}, {upper_str})");
                                    row_entry.push(range_str);
                                }
                                MatchValue::Ternary { value, mask } => {
                                    let value_str = self.convert_data_to_string(key, value);
                                    let mask_str = self.convert_data_to_string(key, mask);
                                    let ternary_str = format!("{value_str} &\n{mask_str}");
                                    row_entry.push(ternary_str);
                                }
                            }
                        }
                    }
                    None => row_entry.push("-".to_string()),
                }
            }
        }
        row_entry
    }

    fn create_action_sub_table(&self, entry: &TableEntry) -> String {
        // Create a sub table containing the action parameters for this action
        let mut action_data_table = Table::new();
        action_data_table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        let mut action_data_header_row: Vec<String> = vec![];
        let mut action_data_row: Vec<String> = vec![];

        for action_data in &entry.action_data {
            action_data_header_row.push(action_data.get_name().to_string());

            let action_data_str =
                self.convert_data_to_string(action_data.get_name(), action_data.get_data());

            action_data_row.push(action_data_str);
        }

        if !action_data_header_row.is_empty() {
            action_data_table.set_titles(Row::from(action_data_header_row));
            action_data_table.add_row(Row::from(action_data_row));
            action_data_table.to_string()
        }
        else {
            "-".to_string()
        }
    }

    /// Prettyprint all given entries as tables.
    /// Table entries need to be fetched from the switch first.
    /// Multiple tables will be printed if entries belong to different tables.
    ///
    /// # Attributes
    /// * `entries` - All entries to print
    ///
    /// # Example
    ///
    /// ```
    /// let tp = PrettyPrinter::new();
    /// let req: table::Request = table::Request::new("ingress.pretty_table");
    /// let res = switch.get_table_entry(req).await?
    /// tp.print_table(res)?;
    /// ```
    pub fn print_table(&self, entries: Vec<TableEntry>) -> Result<(), RBFRTError> {

        // entries might span different tables -> group them by table
        let mut grouped_tables: HashMap<String, Vec<TableEntry>> = HashMap::new();

        for entry in entries {
            let table_name = entry.table_name.clone();
            grouped_tables
                .entry(table_name)
                .or_default()
                .push(entry);
        }

        for (table_name, table_entries) in &grouped_tables {
            let mut table = Table::new();

            let header_row = self.create_header_row(table_entries);
            table.set_titles(Row::from(header_row.clone()));

            for entry in table_entries {
                let row_entry = self.create_data_row(entry, &header_row);
                table.add_row(Row::from(row_entry));
            }

            println!("{:?}:", table_name);
            table.printstd();
            println!();
        }

        Ok(())
    }
}
