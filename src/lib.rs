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

//! This crate implements an interface to a P4-programmable switch that can be controlled through the BFRuntime Interface.
//!
//! Example:
//!
//! ```
//! use rbfrt::{SwitchConnection, table};
//! use rbfrt::table::{MatchValue};
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut switch = SwitchConnection::new("localhost", 50052)
//!         .device_id(0)
//!         .client_id(1)
//!         .p4_name("my_p4_program")
//!         .connect()
//!         .await?;
//!
//!
//!     let requests = vec![
//!         table::Request::new("ingress.p4tg.frame_type.frame_type_monitor")
//!             .match_key("hdr.ipv4.dst_addr", MatchValue::lpm(vec![10, 0, 0, 2], 32))
//!             .match_key("ig_intr_md.ingress_port", MatchValue::exact(0)),
//!         table::Request::new("ingress.p4tg.tg_forward")
//!             .match_key("ig_intr_md.ingress_port", MatchValue::exact(10))
//!             .match_key("ig_md.rand_value", MatchValue::range(20, 30))
//!             .match_key("hdr.pkt_gen.app_id", MatchValue::exact(5))
//!         ];
//!
//!     for req in requests {
//!         let entries = switch.get_table_entry(req).await?;
//!
//!         for e in entries {
//!             println!("{:?}", e);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::{fs, str};

use std::io::Read;

mod protos;
use protos::bfrt_proto;

mod bfrt;
mod core;
pub mod error;
pub mod register;
pub mod table;
pub mod util;

use bfrt::BFRTInfo;
use bfrt_proto::TargetDevice;
use tonic::{Response, Streaming};

use crate::bfrt_proto::{
    ForwardingPipelineConfig, ReadResponse, SetForwardingPipelineConfigRequest,
    StreamMessageRequest, StreamMessageResponse, WriteResponse,
};
use bfrt_proto::bf_runtime_client::BfRuntimeClient;
use bfrt_proto::GetForwardingPipelineConfigRequest;
use table::{Request, RequestType, TableEntry};
use tonic::transport::Channel;

use crate::bfrt_proto::forwarding_pipeline_config::Profile;
use crate::bfrt_proto::set_forwarding_pipeline_config_request::{Action, DevInitMode};
use crate::error::RBFRTError;
use crate::error::RBFRTError::{
    ConnectionError, GRPCError, GetForwardingPipelineError, P4ProgramError, RequestEmpty,
    UnknownReadResult,
};
use crate::register::Register;
use crate::table::MatchValue;

use crate::protos::bfrt_proto::data_field::Value;
use crate::protos::bfrt_proto::stream_message_response::Update;
use crate::util::Digest;
use log::{debug, info, warn};
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;

use crate::protos::bfrt_proto::entity::Entity;
use crate::protos::bfrt_proto::{ReadRequest, WriteRequest};
use crossbeam_channel;

/// Size of the internal digest queue
/// Up to 10k elements with back pressure
const DIGEST_QUEUE_SIZE: usize = 10000;

/// Represents the connection to a switch.
pub struct SwitchConnection {
    ip: String,
    port: u16,
    device_id: u32,
    client_id: u32,
    bf_client: Mutex<BfRuntimeClient<Channel>>,
    bfrt_info: Option<BFRTInfo>,
    target: TargetDevice,
    p4_name: Option<String>,
    send_channel: tokio::sync::mpsc::Sender<StreamMessageRequest>,
    pub digest_queue: crossbeam_channel::Receiver<Digest>,
    config: Option<String>,
}

#[allow(dead_code)]
enum DispatchResult {
    ReadResult {
        response: Response<Streaming<ReadResponse>>,
    },
    WriteResult {
        response: Response<WriteResponse>,
    },
}

pub struct SwitchConnectionBuilder {
    ip: String,
    port: u16,
    device_id: u32,
    client_id: u32,
    p4_name: Option<String>,
    config: Option<String>,
}

impl SwitchConnectionBuilder {
    /// Sets the client id id of the connection
    ///
    /// * `client_id` - ID
    pub fn client_id(mut self, client_id: u32) -> SwitchConnectionBuilder {
        self.client_id = client_id;
        self
    }

    /// Sets the device id of the connection
    ///
    /// * `device_id` - ID
    pub fn device_id(mut self, device_id: u32) -> SwitchConnectionBuilder {
        self.device_id = device_id;
        self
    }

    /// Sets the P4 program name
    ///
    /// * `p4_name` - Name of the P4 program
    pub fn p4_name(mut self, p4_name: &str) -> SwitchConnectionBuilder {
        self.p4_name = Some(p4_name.to_owned());
        self
    }

    /// Sets the path to the Tofino config file
    ///
    /// The config file is used to load a program onto the switch
    ///
    /// * `path` - Path to the config file
    pub fn config(mut self, path: &str) -> SwitchConnectionBuilder {
        self.config = Some(path.to_owned());
        self
    }

    /// Triggers the connection to the switch
    ///
    /// Triggers the connection to the switch and optionally writes a P4 program if `config` is set.
    pub async fn connect(self) -> Result<SwitchConnection, RBFRTError> {
        debug!(
            "Start switch connection to: {}.",
            format!("http://{}:{}", self.ip, self.port)
        );

        match BfRuntimeClient::connect(format!("http://{}:{}", self.ip, self.port)).await {
            Ok(client) => {
                let bf_client = Mutex::new(client);

                let (request_tx, request_rx) =
                    tokio::sync::mpsc::channel::<StreamMessageRequest>(DIGEST_QUEUE_SIZE);
                let (response_tx, mut response_rx) =
                    tokio::sync::mpsc::channel::<StreamMessageResponse>(DIGEST_QUEUE_SIZE);
                let (digest_sender, digest_receiver) =
                    crossbeam_channel::bounded(DIGEST_QUEUE_SIZE);
                let mut connection = SwitchConnection {
                    ip: self.ip,
                    port: self.port,
                    device_id: self.device_id,
                    client_id: self.client_id,
                    bf_client,
                    config: self.config,
                    bfrt_info: None,
                    target: TargetDevice {
                        device_id: self.device_id,
                        pipe_id: 0xffff,
                        direction: 0xff,
                        prsr_id: 0xff,
                    },
                    p4_name: self.p4_name,
                    send_channel: request_tx,
                    digest_queue: digest_receiver,
                };

                if connection.config.is_some() {
                    connection
                        .set_forwarding_pipeline(&connection.config.as_ref().unwrap().clone())
                        .await?;
                }

                if connection.p4_name.is_none() {
                    panic!("P4 name not set.")
                }

                connection
                    .subscribe(request_rx, response_tx, &mut response_rx)
                    .await?;
                connection.bind_forwarding_pipeline().await?;
                connection.bfrt_info = Some(connection.load_pipeline().await?);

                connection.start_notification_thread(response_rx, digest_sender);

                info!(
                    "Switch connection to {} successful.",
                    format!("{}:{}", connection.ip, connection.port)
                );

                Ok(connection)
            }
            Err(e) => Err(ConnectionError {
                ip: self.ip,
                port: self.port,
                orig_e: Box::new(e),
            }),
        }
    }
}

impl SwitchConnection {
    /// Create a new switch connection object
    ///
    /// * `ip` - IP of device
    /// * `port` - GRPC port
    pub fn new(ip: &str, port: u16) -> SwitchConnectionBuilder {
        SwitchConnectionBuilder {
            ip: ip.to_owned(),
            port,
            device_id: 0,
            client_id: 1,
            p4_name: None,
            config: None,
        }
    }

    /// Opens a notification channel.
    /// This is needed to bind to the device and to get notifications from the switch.
    pub async fn subscribe(
        &mut self,
        request_rx: tokio::sync::mpsc::Receiver<StreamMessageRequest>,
        response_tx: tokio::sync::mpsc::Sender<StreamMessageResponse>,
        response_rx: &mut tokio::sync::mpsc::Receiver<StreamMessageResponse>,
    ) -> Result<(), RBFRTError> {
        // subscription request
        let subscribe_req = StreamMessageRequest {
            client_id: self.client_id,
            update: Some(bfrt_proto::stream_message_request::Update::Subscribe(
                bfrt_proto::Subscribe {
                    is_master: true,
                    device_id: self.device_id,
                    notifications: Some(bfrt_proto::subscribe::Notifications {
                        enable_learn_notifications: true,
                        enable_idletimeout_notifications: true,
                        enable_port_status_change_notifications: true,
                    }),
                    status: None,
                },
            )),
        };

        let stream = ReceiverStream::new(request_rx);
        let req = tonic::Request::new(stream);

        let mut clone = { self.bf_client.lock().await.clone() };

        // start thread to listen for notifications
        tokio::spawn(async move {
            let response_channel = clone.stream_channel(req);
            let mut resp = response_channel.await.unwrap().into_inner();

            while let Ok(Some(msg)) = resp.message().await {
                match msg.clone().update.unwrap() {
                    Update::Subscribe(_) | Update::Digest(_) => {
                        let _ = response_tx.try_send(msg);
                    }
                    _ => {
                        warn!(
                            "Got a notification that is currently not supported. Will be ignored."
                        );
                    }
                }
            }

            warn!("Notification channel closed.");
        });

        if self.send_channel.send(subscribe_req).await.is_err() {
            warn!("Notification endpoint hang.")
        }

        let msg = response_rx.recv().await.unwrap();

        match msg.update.unwrap() {
            Update::Subscribe(sub) => {
                if sub.status.unwrap().code != 0 {
                    panic!("Notification subscription failed.");
                } else {
                    info!("Notification subscription successful.")
                }
            }
            _ => {
                panic!("Notification subscription expected.");
            }
        }

        Ok(())
    }

    /// Loads the pipeline information from the switch
    async fn load_pipeline(&mut self) -> Result<BFRTInfo, RBFRTError> {
        debug!("Loading pipeline.");
        match self
            .bf_client
            .lock()
            .await
            .get_forwarding_pipeline_config(GetForwardingPipelineConfigRequest {
                device_id: self.device_id,
                client_id: self.client_id,
            })
            .await
        {
            Ok(pipeline) => {
                let msg = pipeline.into_inner();

                // tofino internal tables
                let non_p4_config = msg.non_p4_config.unwrap();
                let non_p4: BFRTInfo =
                    serde_json::from_slice(&non_p4_config.bfruntime_info).unwrap();
                let non_p4_tables = non_p4.tables();

                for v in msg.config {
                    if v.p4_name == self.p4_name.clone().unwrap() {
                        let mut tmp: BFRTInfo = serde_json::from_slice(&v.bfruntime_info).unwrap();
                        for t in &non_p4_tables {
                            tmp.add_table(t.clone());
                        }

                        return Ok(tmp);
                    }
                }

                Err(P4ProgramError {
                    name: self.p4_name.clone().unwrap(),
                })
            }
            Err(e) => Err(GetForwardingPipelineError {
                device_id: self.device_id,
                client_id: self.client_id,
                orig_e: Box::new(e),
            }),
        }
    }

    fn start_notification_thread(
        &self,
        mut response_rx: tokio::sync::mpsc::Receiver<StreamMessageResponse>,
        digest_queue: crossbeam_channel::Sender<Digest>,
    ) {
        let local_bfrt_info = self.bfrt_info.clone();

        // start receive channel thread
        tokio::spawn(async move {
            let bfrt_info = local_bfrt_info.unwrap();
            while let Some(msg) = response_rx.recv().await {
                match msg.update.unwrap() {
                    Update::Digest(digest) => {
                        let learn_filter = bfrt_info.learn_filter_get(digest.digest_id);

                        // this is really ugly but works at the moment
                        //TODO rewrite
                        match learn_filter {
                            Ok(filter) => {
                                for data in digest.data {
                                    let mut digest_fields = HashMap::new();

                                    for field in data.fields {
                                        let id = field.field_id;
                                        let field_name = filter.get_data_field_name_by_id(id);

                                        if field_name.is_ok() {
                                            let data = field.value;

                                            if data.is_some() {
                                                let data = data.unwrap();

                                                match data {
                                                    Value::Stream(data) => {
                                                        digest_fields
                                                            .insert(field_name.unwrap(), data);
                                                    }
                                                    _ => {
                                                        warn!("Not supported digest field type received.");
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    let digest = Digest {
                                        name: filter.name.to_owned(),
                                        data: digest_fields,
                                    };

                                    let _ = digest_queue.try_send(digest);
                                }
                            }
                            Err(err) => {
                                warn!("Received an error while retrieving learn filter: {}", err);
                            }
                        }
                    }
                    _ => {
                        warn!("Received not supported notification. Only Digests are currently supported.")
                    }
                }
            }

            warn!("Notification channel closed.");
        });
    }

    /// Reads file content and returns byte representation
    ///
    /// * `file_path` - Path to the file
    fn read_file_to_bytes(&self, file_path: &str) -> Vec<u8> {
        let mut file =
            fs::File::open(file_path).unwrap_or_else(|_| panic!("Unable to read: {}", file_path));

        let metadata = fs::metadata(file_path)
            .unwrap_or_else(|_| panic!("Unable to read metadata for {}.", file_path));
        let mut file_buffer = vec![0; metadata.len() as usize];
        file.read(&mut file_buffer).expect("buffer overflow");

        file_buffer
    }

    /// Load a P4 program onto the switch based on the information in the `config_file`
    ///
    /// * `config_file` - Path to the config file (output from P4 compiler)
    async fn set_forwarding_pipeline(&mut self, config_file: &str) -> Result<(), RBFRTError> {
        debug!("Set forwarding pipeline.");

        let file = fs::File::open(config_file)
            .unwrap_or_else(|_| panic!("config file: {} not readable.", config_file));
        let config: core::Configuration =
            serde_json::from_reader(file).expect("config file has invalid json format.");

        let device = config.p4_devices.first().unwrap();

        let mut forwarding_configs: Vec<ForwardingPipelineConfig> = vec![];

        // generate GRPC message
        for program in &device.p4_programs {
            self.p4_name = Some(program.program_name.clone());

            let profiles: Vec<Profile> = program
                .p4_pipelines
                .iter()
                .map(|profile| Profile {
                    profile_name: profile.p4_pipeline_name.to_owned(),
                    context: self.read_file_to_bytes(&profile.context),
                    binary: self.read_file_to_bytes(&profile.config),
                    pipe_scope: profile.pipe_scope.clone(),
                })
                .collect();

            let forwarding_config = ForwardingPipelineConfig {
                p4_name: program.program_name.to_owned(),
                bfruntime_info: self.read_file_to_bytes(&program.bfrt_config),
                profiles,
            };

            forwarding_configs.push(forwarding_config);
        }

        let request = SetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            client_id: self.client_id,
            action: Action::VerifyAndWarmInitBeginAndEnd.into(),
            dev_init_mode: DevInitMode::FastReconfig.into(),
            base_path: "".to_string(),
            config: forwarding_configs,
        };

        let req = self
            .bf_client
            .lock()
            .await
            .set_forwarding_pipeline_config(request)
            .await;

        match req {
            Ok(_) => Ok(()),
            Err(e) => Err(GRPCError {
                message: e.to_string(),
                details: format!("{:?}", e.details()),
            }),
        }
    }

    /// Binds to a P4 program
    async fn bind_forwarding_pipeline(&mut self) -> Result<(), RBFRTError> {
        debug!(
            "Bind forwarding pipeline: {}.",
            self.p4_name.as_ref().unwrap().to_owned()
        );

        let forwarding_config = ForwardingPipelineConfig {
            p4_name: self.p4_name.as_ref().unwrap().to_owned(),
            bfruntime_info: vec![],
            profiles: vec![],
        };

        let request = SetForwardingPipelineConfigRequest {
            device_id: self.device_id,
            client_id: self.client_id,
            action: Action::Bind.into(),
            dev_init_mode: DevInitMode::FastReconfig.into(),
            base_path: "".to_string(),
            config: vec![forwarding_config],
        };

        let req = self
            .bf_client
            .lock()
            .await
            .set_forwarding_pipeline_config(request)
            .await;

        match req {
            Ok(_) => {
                info!("Bind to forwarding pipeline successful.");
                Ok(())
            }
            Err(e) => {
                warn!("Bind forwarding pipeline failed.");
                Err(GRPCError {
                    message: e.to_string(),
                    details: format!("{:?}", e.details()),
                })
            }
        }
    }

    fn get_target_device(&self) -> TargetDevice {
        TargetDevice {
            device_id: self.target.device_id,
            pipe_id: self.target.pipe_id,
            direction: self.target.direction,
            prsr_id: self.target.prsr_id,
        }
    }

    pub async fn get_table_entry(&self, request: Request) -> Result<Vec<TableEntry>, RBFRTError> {
        let entries = self.get_table_entries(vec![request]).await?;

        Ok(entries)
    }

    pub async fn get_table_entries(
        &self,
        requests: Vec<Request>,
    ) -> Result<Vec<TableEntry>, RBFRTError> {
        let mut veq_req = vec![];
        let mut entries = vec![];

        for req in requests {
            veq_req.push(req.request_type(RequestType::Read));
        }

        match self.dispatch_request(&veq_req).await? {
            DispatchResult::ReadResult { response } => {
                let message = response.into_inner().message().await?.unwrap();

                for entity in message.entities {
                    let entity = entity.entity.unwrap();

                    match &entity {
                        Entity::TableEntry(table_entry) => {
                            let table = self
                                .bfrt_info
                                .as_ref()
                                .unwrap()
                                .table_get_by_id(table_entry.table_id)?;

                            let entry = table.parse_read_request(entity, table.name())?;

                            entries.push(entry);
                        }
                        _ => {
                            return Err(UnknownReadResult {});
                        }
                    }
                }

                Ok(entries)
            }
            _ => {
                panic!("Unreachable code.")
            }
        }
    }

    pub async fn write_table_entry(&self, request: Request) -> Result<(), RBFRTError> {
        debug!("Write table entry {}", format!("{:?}", request));

        let req = request.request_type(RequestType::Write);
        let vec_req = vec![req];

        self.dispatch_request(&vec_req).await?;

        Ok(())
    }

    pub async fn write_table_entries(&self, requests: Vec<Request>) -> Result<(), RBFRTError> {
        debug!("Write table entry {}", format!("{:?}", requests));
        let req = requests
            .iter()
            .map(|x| x.clone().request_type(RequestType::Write))
            .collect();
        self.dispatch_request(&req).await?;

        Ok(())
    }

    pub async fn update_table_entry(&self, request: Request) -> Result<(), RBFRTError> {
        debug!("Update table entry {}", format!("{:?}", request));
        let req = request.request_type(RequestType::Update);
        let vec_req = vec![req];
        self.dispatch_request(&vec_req).await?;

        Ok(())
    }

    pub async fn update_table_entries(&self, requests: Vec<Request>) -> Result<(), RBFRTError> {
        debug!("Update table entry {}", format!("{:?}", requests));
        let req = requests
            .iter()
            .map(|x| x.clone().request_type(RequestType::Update))
            .collect();
        self.dispatch_request(&req).await?;

        Ok(())
    }

    pub async fn delete_table_entry(&self, request: Request) -> Result<(), RBFRTError> {
        debug!("Delete table entry {}", format!("{:?}", request));
        let req = request.request_type(RequestType::Delete);

        let vec_req = vec![req];

        self.dispatch_request(&vec_req).await?;

        Ok(())
    }

    pub async fn clear_table(&self, name: &str) -> Result<(), RBFRTError> {
        debug!("Clear table : {}", name);
        let req = Request::new(name);

        self.delete_table_entry(req).await?;

        Ok(())
    }

    pub async fn clear_tables(&self, name: Vec<&str>) -> Result<(), RBFRTError> {
        debug!("Clear tables : {:?}", name);
        let reqs: Vec<Request> = name.iter().map(|x| Request::new(x)).collect();

        self.delete_table_entries(reqs).await?;

        Ok(())
    }

    pub async fn execute_operation(&self, request: Request) -> Result<(), RBFRTError> {
        debug!("Execute operation {}", format!("{:?}", request));
        let req = request.request_type(RequestType::Operation);

        let vec_req = vec![req];

        self.dispatch_request(&vec_req).await?;

        Ok(())
    }

    pub fn has_table(&mut self, name: &str) -> bool {
        let t = self.bfrt_info.as_ref().unwrap().table_get(name);

        t.is_ok()
    }

    pub async fn delete_table_entries(&self, request: Vec<Request>) -> Result<(), RBFRTError> {
        debug!("Delete table entries {}", format!("{:?}", request));
        let vec_req = request
            .iter()
            .map(|x| x.clone().request_type(RequestType::Delete))
            .collect();

        self.dispatch_request(&vec_req).await?;

        Ok(())
    }

    pub async fn get_register_entry(
        &self,
        request: register::Request,
    ) -> Result<Register, RBFRTError> {
        debug!("Read register {}", format!("{:?}", request));
        let mut table_request = Request::new(request.get_name()).request_type(RequestType::Read);

        if request.get_index().is_some() {
            table_request = table_request.match_key(
                "$REGISTER_INDEX",
                MatchValue::exact(request.get_index().unwrap()),
            );
        }

        let entries = self.get_table_entry(table_request).await?;

        let name = request.get_name();

        Ok(Register::parse_register_entries(entries, name))
    }

    pub async fn get_register_entries(
        &self,
        requests: Vec<register::Request>,
    ) -> Result<Register, RBFRTError> {
        debug!("Read register {}", format!("{:?}", requests));

        let name = requests.first().as_ref().unwrap().get_name();

        let mut req = vec![];

        for request in &requests {
            let table_request = Request::new(request.get_name()).request_type(RequestType::Read);

            if request.get_index().is_some() {
                req.push(table_request.match_key(
                    "$REGISTER_INDEX",
                    MatchValue::exact(request.get_index().unwrap()),
                ));
            }
        }

        let entries = self.get_table_entries(req).await?;

        Ok(Register::parse_register_entries(entries, name))
    }

    pub async fn write_register_entry(&self, request: register::Request) -> Result<(), RBFRTError> {
        debug!("Write register {}", format!("{:?}", request));
        let mut table_request = Request::new(request.get_name());

        if request.get_index().is_none() {
            return Err(RBFRTError::MissingRegisterIndex);
        }

        table_request = table_request.match_key(
            "$REGISTER_INDEX",
            MatchValue::exact(request.get_index().unwrap()),
        );

        for (name, value) in request.get_data() {
            table_request = table_request.action_data(name, value.clone());
        }

        self.write_table_entry(table_request).await?;

        Ok(())
    }

    pub async fn write_register_entries(
        &self,
        requests: Vec<register::Request>,
    ) -> Result<(), RBFRTError> {
        debug!("Write register {}", format!("{:?}", requests));

        let mut write_req = vec![];

        for req in &requests {
            if req.get_index().is_none() {
                return Err(RBFRTError::MissingRegisterIndex);
            }

            let mut table_request = Request::new(req.get_name()).match_key(
                "$REGISTER_INDEX",
                MatchValue::exact(req.get_index().unwrap()),
            );

            for (name, value) in req.get_data() {
                table_request = table_request.action_data(name, value.clone());
            }

            write_req.push(table_request);
        }

        self.write_table_entries(write_req).await?;

        Ok(())
    }

    async fn dispatch_request(&self, request: &Vec<Request>) -> Result<DispatchResult, RBFRTError> {
        let bfrt_info = self.bfrt_info.as_ref().unwrap();

        if request.is_empty() {
            return Err(RequestEmpty {});
        }

        match request.first().as_ref().unwrap().get_type() {
            RequestType::Read => {
                let mut entities = vec![];

                for req in request {
                    let table = bfrt_info.table_get(req.get_table_name())?;
                    let entity = table.build_read_request(req, &self.target)?;
                    entities.push(entity);
                }

                let req = ReadRequest {
                    target: Some(self.get_target_device()),
                    client_id: self.client_id,
                    entities,
                    p4_name: self.p4_name.as_ref().unwrap().to_owned(),
                };

                let response = self.bf_client.lock().await.read(req).await?;

                Ok(DispatchResult::ReadResult { response })
            }
            RequestType::Write | RequestType::Update => {
                let mut updates = vec![];

                for req in request {
                    let table = bfrt_info.table_get(req.get_table_name())?;
                    let update = table.build_write_request(req, &self.target)?;
                    updates.push(update);
                }

                let req = WriteRequest {
                    target: Some(self.get_target_device()),
                    client_id: self.client_id,
                    updates,
                    p4_name: self.p4_name.as_ref().unwrap().to_owned(),
                    atomicity: 0,
                };

                let response = self.bf_client.lock().await.write(req).await?;

                Ok(DispatchResult::WriteResult { response })
            }
            RequestType::Operation => {
                let mut updates = vec![];

                for req in request {
                    let table = bfrt_info.table_get(req.get_table_name())?;
                    let update = table.build_operation_request(req)?;
                    updates.push(update);
                }

                let req = WriteRequest {
                    target: Some(self.get_target_device()),
                    client_id: self.client_id,
                    updates,
                    p4_name: self.p4_name.as_ref().unwrap().to_owned(),
                    atomicity: 0,
                };

                let response = self.bf_client.lock().await.write(req).await?;

                Ok(DispatchResult::WriteResult { response })
            }
            RequestType::Delete => {
                let mut updates = vec![];

                for req in request {
                    let table = bfrt_info.table_get(req.get_table_name())?;
                    let update = table.build_delete_request(req, &self.target)?;
                    updates.push(update);
                }

                let req = WriteRequest {
                    target: Some(self.get_target_device()),
                    client_id: self.client_id,
                    updates,
                    p4_name: self.p4_name.as_ref().unwrap().to_owned(),
                    atomicity: 0,
                };

                let response = self.bf_client.lock().await.write(req).await?;

                Ok(DispatchResult::WriteResult { response })
            }
        }
    }
}
