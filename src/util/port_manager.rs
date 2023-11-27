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

use std::collections::{HashMap};
use crate::{SwitchConnection, table};
use crate::error::RBFRTError;
use crate::error::RBFRTError::PortNotFound;
use crate::table::{MatchValue, ToBytes};

use std::{fmt, str};
use strum_macros::EnumString;
use std::str::FromStr;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Speed {
    BF_SPEED_1G,
    BF_SPEED_10G,
    BF_SPEED_20G,
    BF_SPEED_40G,
    BF_SPEED_50G,
    BF_SPEED_100G,
}

#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum AutoNegotiation {
    PM_AN_DEFAULT,
    PM_AN_FORCE_ENABLE,
    PM_AN_FORCE_DISABLE,
}

#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum FEC {
    BF_FEC_TYP_NONE,
    BF_FEC_TYP_FC,
    BF_FEC_TYP_REED_SOLOMON,
}

#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Loopback {
    BF_LPBK_NONE,
    BF_LPBK_MAC_NEAR,
    BF_LPBK_MAC_FAR
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AutoNegotiation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for FEC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Loopback {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Represents a port of a switch
pub struct Port {
    port: u32,
    channel: u8,
    #[serde(rename(serialize = "pid"))]
    dev_port: Option<u32>,
    speed: Speed,
    auto_neg: AutoNegotiation,
    fec: FEC,
    loopback: Loopback,
    enable: bool,
    status: bool
}

impl Port {
    pub fn new(port: u32, channel: u8) -> Port {
        Port {
            port,
            channel,
            dev_port: None,
            speed: Speed::BF_SPEED_1G,
            auto_neg: AutoNegotiation::PM_AN_DEFAULT,
            fec: FEC::BF_FEC_TYP_NONE,
            loopback: Loopback::BF_LPBK_NONE,
            enable: true,
            status: false
        }
    }

    pub fn speed(self, speed: Speed) -> Port {
        Port {speed, ..self}
    }

    pub fn loopback(self, loopback: Loopback) -> Port {
        Port {loopback, ..self}
    }

    pub fn enable(self) -> Port {
        Port {enable: true, ..self}
    }

    pub fn disable(self) -> Port {
        Port {enable: false, ..self}
    }

    pub fn auto_negotiation(self, auto_neg: AutoNegotiation) -> Port {
        Port {auto_neg, ..self}
    }

    pub fn fec(self, fec: FEC) -> Port {
        Port {fec, ..self}
    }

    pub fn get_speed(&self) -> &Speed {
        &self.speed
    }

    pub fn get_loopback(&self) -> &Loopback {
        &self.loopback
    }

    pub fn get_enabled(self) -> bool {
        self.enable
    }

    pub fn get_auto_negotiation(&self) -> &AutoNegotiation {
        &self.auto_neg
    }

    pub fn get_fec(&self) -> &FEC {
        &self.fec
    }

    pub fn get_dev_port(&self) -> Option<u32> {
        self.dev_port
    }

    /// Returns a tuple with (port, channel) of the frontpanel port
    pub fn get_frontpanel_port(&self) -> (u32, u8) {
        (self.port, self.channel)
    }
}
pub struct PortManager {
    mapping_name_to_dev: HashMap<String, u32>,
    mapping_dev_to_name: HashMap<u32, (u32, u8)>
}

impl PortManager {
    pub async fn new(switch: &SwitchConnection) -> PortManager {
        let mut pm = PortManager { mapping_name_to_dev: HashMap::new(), mapping_dev_to_name: HashMap::new() };
        pm.init(switch).await;
        pm
    }

    async fn init(&mut self, switch: &SwitchConnection) {
        match self.do_init(switch).await {
            Ok(_) => {}
            Err(e) => panic!("Error while initializing port manager: {:?}", e)
        }
    }
    async fn do_init(&mut self, switch: &SwitchConnection) -> Result<(), RBFRTError> {
        let req = table::Request::new("$PORT_STR_INFO");
        let all_ports = switch.get_table_entry(req).await?;

        for entry in &all_ports {
            let e = entry.get_action_data("$DEV_PORT")?;
            let port_number = u32::from_be_bytes(e.get_data()[0..4].try_into().unwrap());

            let key = entry.get_key("$PORT_NAME")?;

            let port_name = str::from_utf8(match &key {
                MatchValue::ExactValue { bytes } => {
                    bytes
                }
                _ => panic!("Wrong match value type for port.")
            }).unwrap_or_else(|_| panic!("Error"));

            let port_parts = port_name.split("/").collect::<Vec<&str>>();

            if port_parts.len() == 2 {
                let front_port = port_parts.get(0).unwrap().parse::<u32>().unwrap();
                let channel = port_parts.get(1).unwrap().parse::<u8>().unwrap();

                self.mapping_name_to_dev.insert(port_name.to_owned(), port_number);
                self.mapping_dev_to_name.insert(port_number, (front_port, channel));
            }
        }

        Ok(())
    }

    pub fn dev_port(&self, port: u32, channel:  u8) -> Result<u32, RBFRTError> {
        if self.mapping_name_to_dev.contains_key(&format!("{}/{}", port, channel)) {
            Ok(self.mapping_name_to_dev.get(&format!("{}/{}", port, channel)).unwrap().clone())
        }
        else {
            Err(PortNotFound { name: format!("{}/{}", port, channel)})
        }
    }

    pub fn frontpanel_port(&self, dev_port: u32) -> Result<(u32, u8), RBFRTError> {
        if self.mapping_dev_to_name.contains_key(&dev_port) {
            Ok(self.mapping_dev_to_name.get(&dev_port).unwrap().clone())
        }
        else {
            Err(PortNotFound { name: format!("{}", dev_port)})
        }
    }

    pub async fn add_port(&self, switch: &SwitchConnection, request: &Port) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req =  table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port))
            .action_data("$SPEED", request.speed.to_string())
            .action_data("$FEC", request.fec.to_string())
            .action_data("$PORT_ENABLE", request.enable)
            .action_data("$AUTO_NEGOTIATION", request.auto_neg.to_string())
            .action_data("$LOOPBACK_MODE", request.loopback.to_string());

        switch.write_table_entry(port_req).await?;

        Ok(())
    }

    pub async fn add_ports(&self, switch: &SwitchConnection, requests: &Vec<Port>) -> Result<(), RBFRTError> {
        let all_requests: Result<Vec<table::Request>, RBFRTError> = requests
            .iter()
            .map(|request| {
                let req = table::Request::new("$PORT")
                    .match_key("$DEV_PORT", MatchValue::exact(self.dev_port(request.port, request.channel)?))
                    .action_data("$SPEED", request.speed.to_string())
                    .action_data("$FEC", request.fec.to_string())
                    .action_data("$PORT_ENABLE", request.enable)
                    .action_data("$AUTO_NEGOTIATION", request.auto_neg.to_string())
                    .action_data("$LOOPBACK_MODE", request.loopback.to_string());

                Ok(req)
            }).collect();


        switch.write_table_entries(all_requests?).await?;

        Ok(())
    }

    pub async fn disable_port(&self, switch: &SwitchConnection, request: &Port) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req =  table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port))
            .action_data("$PORT_ENABLE", false);

        switch.update_table_entry(port_req).await?;

        Ok(())
    }

    pub async fn enable_port(&self, switch: &SwitchConnection, request: &Port) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req =  table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port))
            .action_data("$PORT_ENABLE", true);

        switch.update_table_entry(port_req).await?;

        Ok(())
    }

    pub async fn delete_port(&self, switch: &SwitchConnection, request: &Port) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req =  table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port));

        switch.delete_table_entry(port_req).await?;

        Ok(())
    }

    pub async fn clear_ports(&self, switch: &SwitchConnection) -> Result<(), RBFRTError> {
        switch.clear_table("$PORT").await?;

        Ok(())
    }

    pub async fn update_port(&self, switch: &SwitchConnection, request: &Port) -> Result<(), RBFRTError> {
        self.delete_port(switch, request).await?;
        self.add_port(switch, request).await?;

        Ok(())
    }

    /// Returns a list of all currently configured ports.
    ///
    /// * `switch` - SwitchConnection object
    pub async fn get_ports(&self, switch: &SwitchConnection) -> Result<Vec<Port>, RBFRTError> {
        let port_req =  table::Request::new("$PORT");
        let entries = switch.get_table_entry(port_req).await?;

        let mut port_list: Vec<Port> = vec![];

        for e in &entries {
            let key = &e.get_key("$DEV_PORT")?;

            let dev_port = match key {
                MatchValue::ExactValue { bytes } => bytes.to_u32(),
                _ => panic!("Wrong match value for port table.")
            };

            let frontpanel_port = self.frontpanel_port(dev_port)?;

            let mut speed = e.get_action_data("$SPEED")?.get_data().to_string();
            let mut auto_neg = e.get_action_data("$AUTO_NEGOTIATION")?.get_data().to_string();
            let mut fec: String = e.get_action_data("$FEC")?.get_data().to_string();
            let enable = e.get_action_data("$PORT_ENABLE")?.get_data().to_bool();
            let status = e.get_action_data("$PORT_UP")?.get_data().to_bool();
            let mut loopback = e.get_action_data("$LOOPBACK_MODE")?.get_data().to_string();

            // remove strange ascii char e.g., \u17
            fec.retain(|c| c.is_ascii_graphic());
            loopback.retain(|c| c.is_ascii_graphic());
            auto_neg.retain(|c| c.is_ascii_graphic());
            speed.retain(|c| c.is_ascii_graphic());

            let p = Port {
                port: frontpanel_port.0,
                channel: frontpanel_port.1,
                dev_port: Some(dev_port),
                speed: Speed::from_str(speed.trim()).unwrap(),
                auto_neg: AutoNegotiation::from_str(auto_neg.trim()).unwrap(),
                fec: FEC::from_str(&fec).unwrap(),
                enable,
                status,
                loopback: Loopback::from_str(loopback.trim()).unwrap()
            };

            port_list.push(p);
        }

        Ok(port_list)
    }

}