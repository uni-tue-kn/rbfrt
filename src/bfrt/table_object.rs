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

use crate::bfrt::info::Convert;
use crate::bfrt::{BFRTAction, BFRTData, BFRTSingleton, BFRTTableKeyObject, TableType};
use crate::bfrt_proto;
use crate::bfrt_proto::key_field::MatchType;
use crate::bfrt_proto::table_entry::Value;
use crate::bfrt_proto::{
    data_field, entity, key_field, DataField, Entity, KeyField, TableData, TableKey, Update,
};
use crate::error::RBFRTError;
use crate::error::RBFRTError::{
    UnknownActionId, UnknownActionName, UnknownKeyId, UnknownKeyName, UnknownReadResult,
    UnknownSingletonId, UnknownSingletonName,
};
use crate::protos::bfrt_proto::TargetDevice;
use crate::table::{MatchValue, Request, TableEntry, ToBytes};
use prost::Message;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct BFRTTableObject {
    name: String,
    id: u32,
    table_type: TableType,
    #[allow(dead_code)]
    size: u32,
    #[allow(dead_code)]
    has_const_default_action: Option<bool>,
    key: Vec<BFRTTableKeyObject>,
    #[allow(dead_code)]
    action_specs: Option<Vec<BFRTAction>>,
    data: Option<Vec<BFRTData>>,
}

impl BFRTTableObject {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl BFRTTableObject {
    fn build_table_key_data(&self, request: &Request) -> Result<Option<Value>, RBFRTError> {
        let mut val: Option<Value> = None;

        let m = request.get_match_keys();

        if !m.is_empty() {
            let mut key_fields = vec![];

            for entry in m {
                let key = self.get_key_by_name(entry.0)?;
                let key_name = key.name();
                let key_width = key.r#type().get_width();

                let match_type = match &entry.1 {
                    MatchValue::ExactValue { bytes } => MatchType::Exact(key_field::Exact {
                        value: bytes.to_vec().convert(key_name, key_width)?,
                    }),
                    MatchValue::RangeValue {
                        lower_bytes,
                        higher_bytes,
                    } => MatchType::Range(key_field::Range {
                        low: lower_bytes.to_vec().convert(key_name, key_width)?,
                        high: higher_bytes.to_vec().convert(key_name, key_width)?,
                    }),
                    MatchValue::LPM {
                        bytes,
                        prefix_length,
                    } => MatchType::Lpm(key_field::Lpm {
                        value: bytes.to_vec().convert(key_name, key_width)?,
                        prefix_len: prefix_length.to_owned(),
                    }),
                    MatchValue::Ternary { value, mask } => MatchType::Ternary(key_field::Ternary {
                        value: value.to_vec().convert(key_name, key_width)?,
                        mask: mask.to_vec().convert(key_name, key_width)?,
                    }),
                };

                let field = KeyField {
                    field_id: key.id(),
                    match_type: Some(match_type),
                };
                key_fields.push(field);
            }

            val = Some(Value::Key(TableKey { fields: key_fields }));
        }

        Ok(val)
    }

    fn build_table_action_data(&self, request: &Request) -> Result<Option<TableData>, RBFRTError> {
        let mut fields = vec![];

        let id = {
            for entry in request.get_action_data() {
                let (field_width, field_type, field_id) = {
                    // get field width and whether its a singleton or not
                    let singleton = self.get_singleton_by_name(entry.get_key());

                    // check if its associated with an action
                    // that's not the case if it has no action or if its a singleton
                    if !request.has_action() || singleton.is_ok() {
                        let s = singleton?.to_owned();
                        let t = s.clone().get_type().clone().unwrap();
                        (t.get_width(), t.r#type.as_str().to_owned(), s.id())
                    } else {
                        // it should be a regular action data field
                        let action = self.get_action_by_name(request.get_action_name())?;
                        let action_data = action.get_action_data_by_name(entry.get_key())?;
                        let t = action
                            .get_action_data_type(entry.get_key())
                            .unwrap()
                            .clone();
                        (
                            t.get_width(),
                            t.r#type.as_str().to_owned(),
                            action_data.id(),
                        )
                    }
                };

                let value = match field_type.as_str() {
                    "string" => data_field::Value::StrVal(entry.get_data().to_string()),
                    "bool" => data_field::Value::BoolVal(entry.get_data().to_bool()),
                    _ => data_field::Value::Stream(
                        entry
                            .get_data()
                            .clone()
                            .convert(entry.get_key(), field_width)?,
                    ),
                };

                fields.push(DataField {
                    field_id,
                    value: Some(value),
                });
            }

            // do the same for repeated action data fields
            for entry in request.get_action_data_repeated() {
                let (_field_width, field_type, field_id) = {
                    // get field width and whether its a singleton or not
                    let singleton = self.get_singleton_by_name(entry.get_key());

                    // check if its associated with an action
                    // that's not the case if it has no action or if its a singleton
                    if !request.has_action() || singleton.is_ok() {
                        let s = singleton?.to_owned();
                        let t = s.clone().get_type().clone().unwrap();
                        (t.get_width(), t.r#type.as_str().to_owned(), s.id())
                    } else {
                        // it should be a regular action data field
                        let action = self.get_action_by_name(request.get_action_name())?;
                        let action_data = action.get_action_data_by_name(entry.get_key())?;
                        let t = action
                            .get_action_data_type(entry.get_key())
                            .unwrap()
                            .clone();
                        (
                            t.get_width(),
                            t.r#type.as_str().to_owned(),
                            action_data.id(),
                        )
                    }
                };

                let value = match field_type.as_str() {
                    "bool" => {
                        let bool_val = entry.get_data().iter().map(|v| v.to_bool()).collect();
                        let bool_val = data_field::BoolArray { val: bool_val };

                        data_field::Value::BoolArrVal(bool_val)
                    }
                    "uint32" | "uint16" | "uint8" => {
                        let int_val = entry.get_data().iter().map(|v| v.to_u32()).collect();
                        let int_val = data_field::IntArray { val: int_val };

                        data_field::Value::IntArrVal(int_val)
                    }
                    "bytes" => {
                        let vals = entry.get_data().first().unwrap().to_vec();
                        data_field::Value::Stream(vals)
                    }
                    _ => unimplemented!("Not implemented."),
                };

                fields.push(DataField {
                    field_id,
                    value: Some(value),
                });
            }

            if request.has_action() {
                self.get_action_by_name(request.get_action_name())
                    .unwrap()
                    .id
            } else {
                0 // default action id
            }
        };

        let data = Some(TableData {
            action_id: id,
            fields,
        });

        Ok(data)
    }

    #[allow(deprecated)]
    pub fn build_read_request(
        &self,
        request: &Request,
        target: &TargetDevice,
    ) -> Result<Entity, RBFRTError> {
        let val: Option<Value> = self.build_table_key_data(request)?;

        let ent = Entity {
            entity: Some(entity::Entity::TableEntry(bfrt_proto::TableEntry {
                table_id: self.id,
                data: None,
                is_default_entry: false,
                table_read_flag: None,
                table_mod_inc_flag: None,
                entry_tgt: if let Some(pipe) = request.get_pipe() {
                    let mut t = *target;
                    t.pipe_id = pipe;
                    Some(t)
                } else {
                    None
                },
                table_flags: None,
                value: val,
            })),
        };

        Ok(ent)
    }

    #[allow(deprecated)]
    pub fn build_write_request(
        &self,
        request: &Request,
        target: &TargetDevice,
    ) -> Result<Update, RBFRTError> {
        let ent = Entity {
            entity: Some(entity::Entity::TableEntry(bfrt_proto::TableEntry {
                table_id: self.id,
                data: self.build_table_action_data(request)?,
                is_default_entry: request.is_default(),
                table_read_flag: None,
                table_mod_inc_flag: None,
                entry_tgt: if let Some(pipe) = request.get_pipe() {
                    let mut t = *target;
                    t.pipe_id = pipe;
                    Some(t)
                } else {
                    None
                },
                table_flags: None,
                value: self.build_table_key_data(request)?,
            })),
        };

        let mode = match request.get_type() {
            crate::RequestType::Update => 2,
            _ => 1,
        };

        let update = Update {
            r#type: mode,
            entity: Some(ent),
        };

        Ok(update)
    }

    pub fn build_operation_request(&self, request: &Request) -> Result<Update, RBFRTError> {
        let ent = Entity {
            entity: Some(entity::Entity::TableOperation(bfrt_proto::TableOperation {
                table_id: self.id,
                table_operations_type: request.get_operation().get_string(),
            })),
        };

        let update = Update {
            r#type: 1,
            entity: Some(ent),
        };

        Ok(update)
    }

    #[allow(deprecated)]
    pub fn build_delete_request(
        &self,
        request: &Request,
        target: &TargetDevice,
    ) -> Result<Update, RBFRTError> {
        let ent = Entity {
            entity: Some(entity::Entity::TableEntry(bfrt_proto::TableEntry {
                table_id: self.id,
                data: None,
                is_default_entry: false,
                table_read_flag: None,
                table_mod_inc_flag: None,
                entry_tgt: if let Some(pipe) = request.get_pipe() {
                    let mut t = *target;
                    t.pipe_id = pipe;
                    Some(t)
                } else {
                    None
                },
                table_flags: None,
                value: self.build_table_key_data(request)?,
            })),
        };

        let update = Update {
            r#type: 4,
            entity: Some(ent),
        };

        Ok(update)
    }

    pub(crate) fn parse_read_request(
        &self,
        entry: entity::Entity,
        table_name: &str,
    ) -> Result<TableEntry, RBFRTError> {
        match entry {
            entity::Entity::TableEntry(t) => {
                let data = t.data.as_ref().unwrap();

                Ok(TableEntry {
                    table_id: t.table_id,
                    table_name: table_name.to_owned(),
                    match_keys: {
                        let mut match_keys: HashMap<String, MatchValue> = HashMap::new();

                        if let Some(val) = t.value.as_ref() {
                            match val {
                                Value::Key(keys) => {
                                    for k in &keys.fields {
                                        let key = self.get_key_by_id(k.field_id)?;
                                        match_keys.insert(
                                            key.name().to_owned(),
                                            match k.match_type.as_ref().unwrap() {
                                                MatchType::Exact(e) => MatchValue::ExactValue {
                                                    bytes: e.value.clone(),
                                                },
                                                MatchType::Range(r) => MatchValue::RangeValue {
                                                    lower_bytes: r.low.clone(),
                                                    higher_bytes: r.high.clone(),
                                                },
                                                MatchType::Lpm(l) => MatchValue::LPM {
                                                    bytes: l.value.clone(),
                                                    prefix_length: l.prefix_len,
                                                },
                                                MatchType::Ternary(t) => MatchValue::Ternary {
                                                    value: t.value.clone(),
                                                    mask: t.mask.clone(),
                                                },
                                                _ => MatchValue::ExactValue { bytes: vec![] },
                                            },
                                        );
                                    }
                                }
                                Value::HandleId(_) => {}
                            }
                        }

                        match_keys
                    },
                    default_entry: t.is_default_entry,
                    action: match self.get_table_type() {
                        TableType::MatchActionDirect => {
                            let action = self.get_action_by_id(data.action_id)?;
                            String::from(&action.name)
                        }
                        _ => "NoAction".to_owned(),
                    },
                    action_data: {
                        let mut action_data: Vec<crate::table::ActionData> = Vec::new();

                        for f in &data.fields {
                            let key_name =
                                self.get_action_param_name(data.action_id, f.field_id)?;
                            action_data.push(crate::table::ActionData::new(
                                key_name,
                                // convert values to appropriate byte representation
                                match f.value.as_ref().unwrap() {
                                    data_field::Value::Stream(s) => s.to_vec(),
                                    data_field::Value::StrVal(s) => s.encode_to_vec(),
                                    data_field::Value::BoolVal(b) => b.encode_to_vec(),
                                    data_field::Value::FloatVal(f) => f.encode_to_vec(),
                                    data_field::Value::IntArrVal(i) => i.val.clone().to_bytes(),
                                    _ => unimplemented!(
                                        "Not yet implemented. {:?}",
                                        f.value.as_ref().unwrap()
                                    ),
                                },
                            ));
                        }

                        action_data
                    },
                })
            }
            _ => Err(UnknownReadResult {}),
        }
    }

    fn get_action_by_id(&self, action_id: u32) -> Result<&BFRTAction, RBFRTError> {
        for a in self.action_specs.as_ref().unwrap() {
            if a.id == action_id {
                return Ok(a);
            }
        }

        Err(UnknownActionId { action_id })
    }

    pub fn get_action_by_name(&self, name: &str) -> Result<&BFRTAction, RBFRTError> {
        for a in self.action_specs.as_ref().unwrap() {
            if a.name == name {
                return Ok(a);
            }
        }

        Err(UnknownActionName {
            name: name.to_owned(),
        })
    }

    pub fn get_singleton_by_name(&self, name: &str) -> Result<&BFRTSingleton, RBFRTError> {
        for d in self.data.iter().flatten() {
            if d.singleton().name() == name {
                return Ok(d.singleton());
            }
        }

        Err(UnknownSingletonName {
            name: name.to_owned(),
        })
    }

    pub fn get_singleton_by_id(&self, id: u32) -> Result<&BFRTSingleton, RBFRTError> {
        for d in self.data.iter().flatten() {
            if d.singleton().id() == id {
                return Ok(d.singleton());
            }
        }

        Err(UnknownSingletonId { id })
    }

    pub fn get_action_param_name(
        &self,
        action_id: u32,
        param_field_id: u32,
    ) -> Result<&str, RBFRTError> {
        match self.table_type {
            TableType::MatchActionDirect => {
                let action = self.get_action_by_id(action_id)?;
                let action_data = action.get_action_data_by_id(param_field_id);

                match action_data {
                    Ok(data) => Ok(data.name()),
                    Err(e) => match self.get_singleton_by_id(param_field_id) {
                        Ok(s) => Ok(s.name()),
                        Err(_) => Err(e)?,
                    },
                }
            }
            _ => {
                let singleton = self.get_singleton_by_id(param_field_id)?;

                Ok(singleton.name())
            }
        }
    }

    pub fn get_key_by_id(&self, key_id: u32) -> Result<&BFRTTableKeyObject, RBFRTError> {
        for k in &self.key {
            if k.id() == key_id {
                return Ok(k);
            }
        }

        Err(UnknownKeyId {
            id: key_id,
            table_name: self.name.to_owned(),
        })
    }

    pub fn get_key_by_name(&self, name: &str) -> Result<&BFRTTableKeyObject, RBFRTError> {
        for k in &self.key {
            if k.name() == name {
                return Ok(k);
            }
        }

        Err(UnknownKeyName {
            name: name.to_owned(),
            table_name: self.name.to_owned(),
        })
    }

    pub(crate) fn get_table_type(&self) -> &TableType {
        &self.table_type
    }
}
