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

use crate::error::RBFRTError;
use crate::error::RBFRTError::PortNotFound;
use crate::table::{MatchValue, ToBytes};
use crate::{table, SwitchConnection};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{fmt, str};
use strum_macros::EnumString;

/// All possible [Port] speeds to configure.
#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Speed {
    BF_SPEED_1G,
    BF_SPEED_10G,
    BF_SPEED_20G,
    BF_SPEED_25G,
    BF_SPEED_40G,
    BF_SPEED_50G,
    BF_SPEED_100G,
    BF_SPEED_400G,
}

/// All possible auto negotiation options for a [Port].
#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum AutoNegotiation {
    PM_AN_DEFAULT,
    PM_AN_FORCE_ENABLE,
    PM_AN_FORCE_DISABLE,
}

/// All possible forward error correction options for a [Port].
#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum FEC {
    BF_FEC_TYP_NONE,
    BF_FEC_TYP_FC,
    BF_FEC_TYP_REED_SOLOMON,
}

/// All possible loopback options for a [Port].
#[derive(Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Loopback {
    /// No loopback.
    BF_LPBK_NONE,
    /// Loopback from egress to ingress of the same [Port].
    BF_LPBK_MAC_NEAR,
    /// Loopback from ingress to egress of the same [Port].
    BF_LPBK_MAC_FAR,
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

/// Represents a port of the switch.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    status: bool,
}

impl Port {
    /// Creates a new [Port] with the provided frontpanel `port` number and `channel`.
    /// The speed is [1Gb/s](Speed::BF_SPEED_1G), the auto negotiation is [default](AutoNegotiation::PM_AN_DEFAULT), the forward error correction is [disabled](FEC::BF_FEC_TYP_NONE), and the loopback is [disabled](Loopback::BF_LPBK_NONE).
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
            status: false,
        }
    }

    /// Sets the speed.
    pub fn speed(self, speed: Speed) -> Port {
        Port { speed, ..self }
    }

    /// Sets the loopback mode.
    pub fn loopback(self, loopback: Loopback) -> Port {
        Port { loopback, ..self }
    }

    /// Enables the [Port].
    pub fn enable(self) -> Port {
        Port {
            enable: true,
            ..self
        }
    }

    /// Disables the [Port].
    pub fn disable(self) -> Port {
        Port {
            enable: false,
            ..self
        }
    }

    /// Sets the auto negotiation.
    pub fn auto_negotiation(self, auto_neg: AutoNegotiation) -> Port {
        Port { auto_neg, ..self }
    }

    /// Sets the forward error correction.
    pub fn fec(self, fec: FEC) -> Port {
        Port { fec, ..self }
    }

    /// Returns the configured speed.
    pub fn get_speed(&self) -> &Speed {
        &self.speed
    }

    /// Returns the configured loopback mode.
    pub fn get_loopback(&self) -> &Loopback {
        &self.loopback
    }

    /// Returns if the [Port] is enabled.
    pub fn get_enabled(self) -> bool {
        self.enable
    }

    /// Returns the configured auto negotiation.
    pub fn get_auto_negotiation(&self) -> &AutoNegotiation {
        &self.auto_neg
    }

    /// Returns the configured forward error correction.
    pub fn get_fec(&self) -> &FEC {
        &self.fec
    }

    /// Returns the `dev_port` of the frontpanel `port` number.
    pub fn get_dev_port(&self) -> Option<u32> {
        self.dev_port
    }

    /// Returns a tuple `(port, channel)` of the frontpanel port.
    pub fn get_frontpanel_port(&self) -> (u32, u8) {
        (self.port, self.channel)
    }
}

/// Manager to add, update, delete, ... [Ports](Port) of the connected switch.
///
/// # Example
///
/// ```no_run
/// use rbfrt::{SwitchConnection};
/// use rbfrt::util::{Port, Speed, FEC, AutoNegotiation, Loopback, PortManager};
///
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let switch = SwitchConnection::builder("localhost", 50052)
///         .device_id(0)
///         .client_id(1)
///         .p4_name("my_p4_program")
///         .connect()
///         .await?;
///
///     let port = Port::new(1, 0)
///         .speed(Speed::BF_SPEED_100G)
///         .fec(FEC::BF_FEC_TYP_NONE)
///         .auto_negotiation(AutoNegotiation::PM_AN_DEFAULT)
///         .loopback(Loopback::BF_LPBK_MAC_NEAR);
///
///     let pm = PortManager::new(&switch).await;
///     pm.add_port(&switch, &port);
///
///     Ok(())
/// }
/// ```
pub struct PortManager {
    mapping_name_to_dev: HashMap<String, u32>,
    mapping_dev_to_name: HashMap<u32, (u32, u8)>,
}

impl PortManager {
    pub async fn new(switch: &SwitchConnection) -> PortManager {
        let mut pm = PortManager {
            mapping_name_to_dev: HashMap::new(),
            mapping_dev_to_name: HashMap::new(),
        };
        pm.init(switch).await;
        pm
    }

    async fn init(&mut self, switch: &SwitchConnection) {
        match self.do_init(switch).await {
            Ok(_) => {}
            Err(e) => panic!("Error while initializing port manager: {:?}", e),
        }
    }
    async fn do_init(&mut self, switch: &SwitchConnection) -> Result<(), RBFRTError> {
        let req = table::Request::new("$PORT_STR_INFO");
        let all_ports = switch.get_table_entries(req).await?;

        for entry in &all_ports {
            let e = entry.get_action_data("$DEV_PORT")?;
            let port_number = u32::from_be_bytes(e.get_data()[0..4].try_into().unwrap());

            let key = entry.get_key("$PORT_NAME")?;

            let port_name = str::from_utf8(match &key {
                MatchValue::ExactValue { bytes } => bytes,
                _ => panic!("Wrong match value type for port."),
            })
            .unwrap_or_else(|_| panic!("Error"));

            let port_parts = port_name.split('/').collect::<Vec<&str>>();

            if port_parts.len() == 2 {
                let front_port = port_parts.first().unwrap().parse::<u32>().unwrap();
                let channel = port_parts.get(1).unwrap().parse::<u8>().unwrap();

                self.mapping_name_to_dev
                    .insert(port_name.to_owned(), port_number);
                self.mapping_dev_to_name
                    .insert(port_number, (front_port, channel));
            }
        }

        Ok(())
    }

    /// Returns a list of all configured ports of the connected `switch`.
    pub async fn get_ports(&self, switch: &SwitchConnection) -> Result<Vec<Port>, RBFRTError> {
        let port_req = table::Request::new("$PORT");
        let entries = switch.get_table_entries(port_req).await?;

        let mut port_list: Vec<Port> = vec![];

        for e in &entries {
            let key = &e.get_key("$DEV_PORT")?;

            let dev_port = match key {
                MatchValue::ExactValue { bytes } => bytes.to_u32(),
                _ => panic!("Wrong match value for port table."),
            };

            let frontpanel_port = self.frontpanel_port(dev_port)?;

            let mut speed = e.get_action_data("$SPEED")?.get_data().to_string();
            let mut auto_neg = e
                .get_action_data("$AUTO_NEGOTIATION")?
                .get_data()
                .to_string();
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
                loopback: Loopback::from_str(loopback.trim()).unwrap(),
            };

            port_list.push(p);
        }

        Ok(port_list)
    }

    /// Configures the provided [Port].
    pub async fn add_port(
        &self,
        switch: &SwitchConnection,
        request: &Port,
    ) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req = table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port))
            .action_data("$SPEED", request.speed.to_string())
            .action_data("$FEC", request.fec.to_string())
            .action_data("$PORT_ENABLE", request.enable)
            .action_data("$AUTO_NEGOTIATION", request.auto_neg.to_string())
            .action_data("$LOOPBACK_MODE", request.loopback.to_string());

        switch.write_table_entry(port_req).await?;

        Ok(())
    }

    /// Configures all provided [Ports](Port).
    pub async fn add_ports(
        &self,
        switch: &SwitchConnection,
        requests: &[Port],
    ) -> Result<(), RBFRTError> {
        let all_requests: Result<Vec<table::Request>, RBFRTError> = requests
            .iter()
            .map(|request| {
                let req = table::Request::new("$PORT")
                    .match_key(
                        "$DEV_PORT",
                        MatchValue::exact(self.dev_port(request.port, request.channel)?),
                    )
                    .action_data("$SPEED", request.speed.to_string())
                    .action_data("$FEC", request.fec.to_string())
                    .action_data("$PORT_ENABLE", request.enable)
                    .action_data("$AUTO_NEGOTIATION", request.auto_neg.to_string())
                    .action_data("$LOOPBACK_MODE", request.loopback.to_string());

                Ok(req)
            })
            .collect();

        switch.write_table_entries(all_requests?).await?;

        Ok(())
    }

    /// Deletes the already configured [Port] and adds it with the new configuration.
    pub async fn update_port(
        &self,
        switch: &SwitchConnection,
        request: &Port,
    ) -> Result<(), RBFRTError> {
        self.delete_port(switch, request).await?;
        self.add_port(switch, request).await?;

        Ok(())
    }

    /// Deletes the [Port] with the specified frontpanel port number.
    pub async fn delete_port(
        &self,
        switch: &SwitchConnection,
        request: &Port,
    ) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req =
            table::Request::new("$PORT").match_key("$DEV_PORT", MatchValue::exact(dev_port));

        switch.delete_table_entry(port_req).await?;

        Ok(())
    }

    /// Deletes all configured [Ports](Port).
    pub async fn clear_ports(&self, switch: &SwitchConnection) -> Result<(), RBFRTError> {
        switch.clear_table("$PORT").await?;

        Ok(())
    }

    /// Enables the [Port] with the provided frontpanel port number.
    pub async fn enable_port(
        &self,
        switch: &SwitchConnection,
        request: &Port,
    ) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req = table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port))
            .action_data("$PORT_ENABLE", true);

        switch.update_table_entry(port_req).await?;

        Ok(())
    }

    /// Disables the [Port] with the provided frontpanel port number.
    pub async fn disable_port(
        &self,
        switch: &SwitchConnection,
        request: &Port,
    ) -> Result<(), RBFRTError> {
        let dev_port = self.dev_port(request.port, request.channel)?;

        let port_req = table::Request::new("$PORT")
            .match_key("$DEV_PORT", MatchValue::exact(dev_port))
            .action_data("$PORT_ENABLE", false);

        switch.update_table_entry(port_req).await?;

        Ok(())
    }

    /// Maps the frontpanel port to the switch interal `dev_port`.
    pub fn dev_port(&self, port: u32, channel: u8) -> Result<u32, RBFRTError> {
        if self
            .mapping_name_to_dev
            .contains_key(&format!("{}/{}", port, channel))
        {
            Ok(*self
                .mapping_name_to_dev
                .get(&format!("{}/{}", port, channel))
                .unwrap())
        } else {
            Err(PortNotFound {
                name: format!("{}/{}", port, channel),
            })
        }
    }

    /// Returns a tuple `(frontpanel port, channel)` of the provided `dev_port`.
    pub fn frontpanel_port(&self, dev_port: u32) -> Result<(u32, u8), RBFRTError> {
        if self.mapping_dev_to_name.contains_key(&dev_port) {
            Ok(*self.mapping_dev_to_name.get(&dev_port).unwrap())
        } else {
            Err(PortNotFound {
                name: format!("{}", dev_port),
            })
        }
    }
}
