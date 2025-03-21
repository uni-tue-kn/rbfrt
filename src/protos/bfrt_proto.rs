/// ------------------------------------------------------------------------------
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    /// This is the default TargetDevice.
    /// If entry_tgt under TableEntry is specified, that takes precedence over this field
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<TargetDevice>,
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
    /// The write batch, comprising a list of Update operations.
    #[prost(message, repeated, tag = "3")]
    pub updates: ::prost::alloc::vec::Vec<Update>,
    #[prost(enumeration = "write_request::Atomicity", tag = "4")]
    pub atomicity: i32,
    #[prost(string, tag = "5")]
    pub p4_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `WriteRequest`.
pub mod write_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Atomicity {
        /// Required. This is the default behavior. The batch is processed in a
        /// non-atomic manner from a dataplane point of view. Each operation within
        /// the batch must be attempted even if one or more encounter errors.
        /// Every dataplane packet is guaranteed to be processed according to
        /// table contents as they are between two individual operations of the
        /// batch, but there could be several packets processed that see each of
        /// these intermediate stages.
        ContinueOnError = 0,
        /// Optional. Operations within the batch are committed to dataplane until
        /// an error is encountered. At this point, the operations must be rolled
        /// back such that both software and dataplane state is consistent with the
        /// state before the batch was attempted. The resulting behavior is
        /// all-or-none, except the batch is not atomic from a data plane point of
        /// view. Every dataplane packet is guaranteed to be processed according to
        /// table contents as they are between two individual operations of the
        /// batch, but there could be several packets processed that see each of
        /// these intermediate stages.
        RollbackOnError = 1,
        /// Optional. Every dataplane packet is guaranteed to be processed according
        /// to table contents before the batch began, or after the batch completed
        /// and the operations were programmed to the hardware.
        /// The batch is therefore treated as a transaction.
        DataplaneAtomic = 2,
    }
    impl Atomicity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Atomicity::ContinueOnError => "CONTINUE_ON_ERROR",
                Atomicity::RollbackOnError => "ROLLBACK_ON_ERROR",
                Atomicity::DataplaneAtomic => "DATAPLANE_ATOMIC",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    #[prost(message, repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// This is the default TargetDevice.
    /// If entry_tgt under TableEntry is specified, that takes precedence over this field
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<TargetDevice>,
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    #[prost(string, tag = "4")]
    pub p4_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag = "2")]
    pub status: ::prost::alloc::vec::Vec<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetDevice {
    #[prost(uint32, tag = "1")]
    pub device_id: u32,
    #[prost(uint32, tag = "2")]
    pub pipe_id: u32,
    #[prost(uint32, tag = "3")]
    pub direction: u32,
    /// More target-specific ids.
    #[prost(uint32, tag = "4")]
    pub prsr_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    #[prost(enumeration = "update::Type", tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub entity: ::core::option::Option<Entity>,
}
/// Nested message and enum types in `Update`.
pub mod update {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        Insert = 1,
        Modify = 2,
        /// MODIFY_INC is used to add/delete the given data to/from the
        /// existing table entry incrementally.
        ModifyInc = 3,
        Delete = 4,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "UNSPECIFIED",
                Type::Insert => "INSERT",
                Type::Modify => "MODIFY",
                Type::ModifyInc => "MODIFY_INC",
                Type::Delete => "DELETE",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(oneof = "entity::Entity", tags = "1, 2, 3, 4, 5, 6")]
    pub entity: ::core::option::Option<entity::Entity>,
}
/// Nested message and enum types in `Entity`.
pub mod entity {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entity {
        #[prost(message, tag = "1")]
        TableEntry(super::TableEntry),
        #[prost(message, tag = "2")]
        TableUsage(super::TableUsage),
        #[prost(message, tag = "3")]
        TableAttribute(super::TableAttribute),
        #[prost(message, tag = "4")]
        TableOperation(super::TableOperation),
        #[prost(message, tag = "5")]
        ObjectId(super::ObjectId),
        #[prost(message, tag = "6")]
        Handle(super::HandleId),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleId {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(oneof = "handle_id::Value", tags = "2, 3")]
    pub value: ::core::option::Option<handle_id::Value>,
}
/// Nested message and enum types in `HandleId`.
pub mod handle_id {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "2")]
        Key(super::TableKey),
        #[prost(uint32, tag = "3")]
        HandleId(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableEntry {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<TableData>,
    #[prost(bool, tag = "4")]
    pub is_default_entry: bool,
    /// Deprecated, please use table_flags
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub table_read_flag: ::core::option::Option<TableReadFlag>,
    #[deprecated]
    #[prost(message, optional, tag = "6")]
    pub table_mod_inc_flag: ::core::option::Option<TableModIncFlag>,
    /// If entry_tgt is specified, all the fields of entry_tgt are used even if not explicitly set
    #[prost(message, optional, tag = "8")]
    pub entry_tgt: ::core::option::Option<TargetDevice>,
    #[prost(message, optional, tag = "9")]
    pub table_flags: ::core::option::Option<TableFlags>,
    #[prost(oneof = "table_entry::Value", tags = "2, 7")]
    pub value: ::core::option::Option<table_entry::Value>,
}
/// Nested message and enum types in `TableEntry`.
pub mod table_entry {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "2")]
        Key(super::TableKey),
        #[prost(uint32, tag = "7")]
        HandleId(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableUsage {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(uint32, tag = "2")]
    pub usage: u32,
    /// Deprecated, please use table_flags
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub table_read_flag: ::core::option::Option<TableReadFlag>,
    #[prost(message, optional, tag = "4")]
    pub table_flags: ::core::option::Option<TableFlags>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableAttribute {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(oneof = "table_attribute::Attribute", tags = "2, 3, 4, 5, 6, 7, 8, 9")]
    pub attribute: ::core::option::Option<table_attribute::Attribute>,
}
/// Nested message and enum types in `TableAttribute`.
pub mod table_attribute {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Attribute {
        #[prost(message, tag = "2")]
        IdleTable(super::IdleTable),
        #[prost(message, tag = "3")]
        EntryScope(super::EntryScope),
        #[prost(message, tag = "4")]
        DynKeyMask(super::DynKeyMask),
        #[prost(message, tag = "5")]
        DynHashing(super::DynHashing),
        #[prost(message, tag = "6")]
        ByteCountAdj(super::ByteCountAdj),
        #[prost(message, tag = "7")]
        PortStatusNotify(super::PortStatusChg),
        #[prost(message, tag = "8")]
        IntvlMs(super::StatePullIntvl),
        #[prost(message, tag = "9")]
        PreDeviceConfig(super::PreDeviceConfig),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableOperation {
    #[prost(uint32, tag = "1")]
    pub table_id: u32,
    #[prost(string, tag = "2")]
    pub table_operations_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableData {
    #[prost(uint32, tag = "1")]
    pub action_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<DataField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataField {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    /// All data fields are dealt with using a byte stream except for float
    /// values. Float values are used for data fields for LPF and WRED table
    #[prost(oneof = "data_field::Value", tags = "2, 3, 4, 5, 6, 7, 8, 9")]
    pub value: ::core::option::Option<data_field::Value>,
}
/// Nested message and enum types in `DataField`.
pub mod data_field {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntArray {
        #[prost(uint32, repeated, tag = "1")]
        pub val: ::prost::alloc::vec::Vec<u32>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BoolArray {
        #[prost(bool, repeated, tag = "1")]
        pub val: ::prost::alloc::vec::Vec<bool>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StrArray {
        #[prost(string, repeated, tag = "1")]
        pub val: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContainerArray {
        #[prost(message, repeated, tag = "1")]
        pub container: ::prost::alloc::vec::Vec<container_array::Container>,
    }
    /// Nested message and enum types in `ContainerArray`.
    pub mod container_array {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Container {
            #[prost(message, repeated, tag = "1")]
            pub val: ::prost::alloc::vec::Vec<super::super::DataField>,
        }
    }
    /// All data fields are dealt with using a byte stream except for float
    /// values. Float values are used for data fields for LPF and WRED table
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(bytes, tag = "2")]
        Stream(::prost::alloc::vec::Vec<u8>),
        #[prost(float, tag = "3")]
        FloatVal(f32),
        #[prost(string, tag = "4")]
        StrVal(::prost::alloc::string::String),
        #[prost(message, tag = "5")]
        IntArrVal(IntArray),
        #[prost(message, tag = "6")]
        BoolArrVal(BoolArray),
        #[prost(message, tag = "7")]
        ContainerArrVal(ContainerArray),
        #[prost(bool, tag = "8")]
        BoolVal(bool),
        #[prost(message, tag = "9")]
        StrArrVal(StrArray),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableKey {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<KeyField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyField {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    #[prost(oneof = "key_field::MatchType", tags = "2, 3, 4, 5, 6")]
    pub match_type: ::core::option::Option<key_field::MatchType>,
}
/// Nested message and enum types in `KeyField`.
pub mod key_field {
    /// Matches can be performed on arbitrarily-large inputs; the protobuf type
    /// 'bytes' is used to model arbitrarily-large values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Exact {
        #[prost(bytes = "vec", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Ternary {
        #[prost(bytes = "vec", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub mask: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Lpm {
        #[prost(bytes = "vec", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        /// in bits
        #[prost(int32, tag = "2")]
        pub prefix_len: i32,
    }
    /// A Range is logically a set that contains all values numerically between
    /// 'low' and 'high' inclusively.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        #[prost(bytes = "vec", tag = "1")]
        pub low: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes = "vec", tag = "2")]
        pub high: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Optional {
        #[prost(bytes = "vec", tag = "1")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(bool, tag = "2")]
        pub is_valid: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchType {
        #[prost(message, tag = "2")]
        Exact(Exact),
        #[prost(message, tag = "3")]
        Ternary(Ternary),
        #[prost(message, tag = "4")]
        Lpm(Lpm),
        #[prost(message, tag = "5")]
        Range(Range),
        #[prost(message, tag = "6")]
        Optional(Optional),
    }
}
/// Deprecated, please use TableFlags
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableReadFlag {
    #[prost(bool, tag = "1")]
    pub from_hw: bool,
    #[prost(bool, tag = "2")]
    pub key_only: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableFlags {
    #[prost(bool, tag = "1")]
    pub from_hw: bool,
    #[prost(bool, tag = "2")]
    pub key_only: bool,
    #[prost(bool, tag = "3")]
    pub mod_del: bool,
    #[prost(bool, tag = "4")]
    pub reset_ttl: bool,
}
/// Deprecated, please use TableFlags
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableModIncFlag {
    #[prost(enumeration = "table_mod_inc_flag::Type", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `TableModIncFlag`.
pub mod table_mod_inc_flag {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Enum to add the given data incrementally to the
        /// exising table entry
        ModIncAdd = 0,
        /// Enum to delete the given data from the
        /// exising table entry
        ModIncDelete = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::ModIncAdd => "MOD_INC_ADD",
                Type::ModIncDelete => "MOD_INC_DELETE",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyFieldMask {
    #[prost(uint32, tag = "1")]
    pub field_id: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub mask: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynKeyMask {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<KeyFieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynHashing {
    #[prost(uint32, tag = "1")]
    pub alg: u32,
    #[prost(uint64, tag = "2")]
    pub seed: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ByteCountAdj {
    #[prost(int32, tag = "1")]
    pub byte_count_adjust: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdleTable {
    #[prost(uint32, tag = "1")]
    pub ttl_query_interval: u32,
    #[prost(uint32, tag = "2")]
    pub max_ttl: u32,
    #[prost(uint32, tag = "3")]
    pub min_ttl: u32,
    #[prost(enumeration = "idle_table::IdleTableMode", tag = "4")]
    pub idle_table_mode: i32,
    #[prost(bool, tag = "5")]
    pub enable: bool,
}
/// Nested message and enum types in `IdleTable`.
pub mod idle_table {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdleTableMode {
        IdleTablePollMode = 0,
        IdleTableNotifyMode = 1,
    }
    impl IdleTableMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IdleTableMode::IdleTablePollMode => "IDLE_TABLE_POLL_MODE",
                IdleTableMode::IdleTableNotifyMode => "IDLE_TABLE_NOTIFY_MODE",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatePullIntvl {
    #[prost(uint32, tag = "1")]
    pub intvl_val: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortStatusChg {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mode {
    #[prost(uint32, tag = "3")]
    pub args: u32,
    #[prost(oneof = "mode::Scope", tags = "1, 2")]
    pub scope: ::core::option::Option<mode::Scope>,
}
/// Nested message and enum types in `Mode`.
pub mod mode {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PredefinedMode {
        All = 0,
        Single = 1,
    }
    impl PredefinedMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PredefinedMode::All => "ALL",
                PredefinedMode::Single => "SINGLE",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        #[prost(enumeration = "PredefinedMode", tag = "1")]
        Predef(i32),
        #[prost(uint32, tag = "2")]
        UserDefined(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreGlobalRid {
    #[prost(uint32, tag = "1")]
    pub global_rid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrePortProtection {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreFastFailover {
    #[prost(bool, tag = "1")]
    pub enable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreMaxNodesBeforeYield {
    #[prost(uint32, tag = "1")]
    pub count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreMaxNodeThreshold {
    #[prost(uint32, tag = "1")]
    pub node_count: u32,
    #[prost(uint32, tag = "2")]
    pub port_lag_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreDeviceConfig {
    #[prost(message, optional, tag = "1")]
    pub pre_global_rid: ::core::option::Option<PreGlobalRid>,
    #[prost(message, optional, tag = "2")]
    pub pre_port_protection: ::core::option::Option<PrePortProtection>,
    #[prost(message, optional, tag = "3")]
    pub pre_fast_failover: ::core::option::Option<PreFastFailover>,
    #[prost(message, optional, tag = "4")]
    pub pre_max_nodes_before_yield: ::core::option::Option<PreMaxNodesBeforeYield>,
    #[prost(message, optional, tag = "5")]
    pub pre_max_node_threshold: ::core::option::Option<PreMaxNodeThreshold>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryScope {
    #[prost(message, optional, tag = "1")]
    pub gress_scope: ::core::option::Option<Mode>,
    #[prost(message, optional, tag = "2")]
    pub pipe_scope: ::core::option::Option<Mode>,
    #[prost(message, optional, tag = "3")]
    pub prsr_scope: ::core::option::Option<Mode>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectId {
    #[prost(uint32, tag = "3")]
    pub id: u32,
    #[prost(oneof = "object_id::Object", tags = "1, 2")]
    pub object: ::core::option::Option<object_id::Object>,
}
/// Nested message and enum types in `ObjectId`.
pub mod object_id {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionName {
        #[prost(string, tag = "1")]
        pub action: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyFieldName {
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataFieldName {
        #[prost(string, tag = "1")]
        pub action: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub field: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableObject {
        #[prost(string, tag = "1")]
        pub table_name: ::prost::alloc::string::String,
        #[prost(oneof = "table_object::Names", tags = "2, 3, 4")]
        pub names: ::core::option::Option<table_object::Names>,
    }
    /// Nested message and enum types in `TableObject`.
    pub mod table_object {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Names {
            #[prost(message, tag = "2")]
            ActionName(super::ActionName),
            #[prost(message, tag = "3")]
            KeyFieldName(super::KeyFieldName),
            #[prost(message, tag = "4")]
            DataFieldName(super::DataFieldName),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LearnObject {
        #[prost(string, tag = "1")]
        pub learn_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub data_field_name: ::core::option::Option<DataFieldName>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Object {
        #[prost(message, tag = "1")]
        TableObject(TableObject),
        #[prost(message, tag = "2")]
        LearnObject(LearnObject),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMessageRequest {
    #[prost(uint32, tag = "1")]
    pub client_id: u32,
    #[prost(oneof = "stream_message_request::Update", tags = "2, 3")]
    pub update: ::core::option::Option<stream_message_request::Update>,
}
/// Nested message and enum types in `StreamMessageRequest`.
pub mod stream_message_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        #[prost(message, tag = "2")]
        Subscribe(super::Subscribe),
        #[prost(message, tag = "3")]
        DigestAck(super::DigestListAck),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscribe {
    #[deprecated]
    #[prost(bool, tag = "1")]
    pub is_master: bool,
    /// Master for Warm Init messages.
    /// Deprecated and not needed anymore.
    /// Keeping for backward compatibility.
    ///
    /// Device ID
    #[prost(uint32, tag = "2")]
    pub device_id: u32,
    /// Contains which notifications need to be
    #[prost(message, optional, tag = "3")]
    pub notifications: ::core::option::Option<subscribe::Notifications>,
    /// enabled for this client. Default value of
    /// these notifications are false.
    ///
    /// The controller doesn't populate this field.
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<super::google_rpc::Status>,
}
/// Nested message and enum types in `Subscribe`.
pub mod subscribe {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Notifications {
        /// Enable learn digest notifications. These notifications are
        #[prost(bool, tag = "1")]
        pub enable_learn_notifications: bool,
        /// (device, P4-program) based so these will be triggered only after a
        /// client binds to a program.
        ///
        /// Enable idletimeout notifications. These are on per table basis and
        #[prost(bool, tag = "2")]
        pub enable_idletimeout_notifications: bool,
        /// hence (device, P4-Program) based so these will be triggered only
        /// after a client binds to a program.
        ///
        /// Enable port status change notifications. These notifications are
        #[prost(bool, tag = "3")]
        pub enable_port_status_change_notifications: bool,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DigestListAck {
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[prost(uint32, tag = "2")]
    pub list_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamMessageResponse {
    #[prost(oneof = "stream_message_response::Update", tags = "1, 2, 3, 4, 5")]
    pub update: ::core::option::Option<stream_message_response::Update>,
}
/// Nested message and enum types in `StreamMessageResponse`.
pub mod stream_message_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        /// This message is only used to let the server know
        #[prost(message, tag = "1")]
        Subscribe(super::Subscribe),
        /// of the existence of client with this client_id
        ///
        /// Learn Digest
        #[prost(message, tag = "2")]
        Digest(super::DigestList),
        /// Idle timeout notification
        #[prost(message, tag = "3")]
        IdleTimeoutNotification(super::IdleTimeoutNotification),
        /// Port status change notification
        #[prost(message, tag = "4")]
        PortStatusChangeNotification(super::PortStatusChgNotification),
        /// Response for a SetForwardingPipelineConfigRequest is sent here
        #[prost(message, tag = "5")]
        SetForwardingPipelineConfigResponse(super::SetForwardingPipelineConfigResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DigestList {
    /// Identifies the digest extern instance
    #[prost(uint32, tag = "1")]
    pub digest_id: u32,
    #[prost(uint32, tag = "2")]
    pub list_id: u32,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<TableData>,
    #[prost(message, optional, tag = "4")]
    pub target: ::core::option::Option<TargetDevice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdleTimeoutNotification {
    /// Only "key" fields are required to be set in each TableEntry.
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<TargetDevice>,
    #[prost(message, optional, tag = "2")]
    pub table_entry: ::core::option::Option<TableEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortStatusChgNotification {
    /// Only "key" fields are required to be set in each TableEntry.
    #[prost(message, optional, tag = "1")]
    pub table_entry: ::core::option::Option<TableEntry>,
    #[prost(bool, tag = "2")]
    pub port_up: bool,
}
/// -----------------------------------------------------------------------------
/// SetForwardingPipelineConfig RPC takes in this message. It should contain
/// details of the entire device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetForwardingPipelineConfigRequest {
    /// Device ID
    #[prost(uint32, tag = "1")]
    pub device_id: u32,
    /// Client ID
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
    /// action
    #[prost(
        enumeration = "set_forwarding_pipeline_config_request::Action",
        tag = "3"
    )]
    pub action: i32,
    /// warm init mode. Fast reconfig or Hitless
    #[prost(
        enumeration = "set_forwarding_pipeline_config_request::DevInitMode",
        tag = "4"
    )]
    pub dev_init_mode: i32,
    /// The base path where the config is wished to be
    #[prost(string, tag = "5")]
    pub base_path: ::prost::alloc::string::String,
    /// stored. If empty, then current directory is used
    ///
    /// Device's config
    #[prost(message, repeated, tag = "6")]
    pub config: ::prost::alloc::vec::Vec<ForwardingPipelineConfig>,
}
/// Nested message and enum types in `SetForwardingPipelineConfigRequest`.
pub mod set_forwarding_pipeline_config_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// BIND: Default Action. Only binds the client to the program
        Bind = 0,
        /// specified in the p4_name. One client can bind to only one
        /// program. One program can have only one client as of now. Even
        /// in case of multiple programs on a single device, BIND requires
        /// just one program’s config msg. If multiple repeated
        /// forwarding_pipeline_config msgs are sent as part of this
        /// request, then google.rpc.INVALID_ARGUMENT is sent. If a client
        /// doesn't BIND, then it can only access
        /// SetForwardingPipelineConfigRequest,
        /// GetForwardingPipelineConfigRequest and StreamMessageRequest
        /// RPCs. Read and Write RPCs are not allowed for non-bound clients
        ///
        /// VERIFY(Master): Verifies whether this config is valid or not.
        Verify = 1,
        /// Upon failure or incomplete config in the msg,
        /// google.rpc.Code::INVALID_ARGUMENT is sent.
        ///
        /// VERIFY_AND_WARM_INIT_BEGIN(Master):  Verifies the config and then
        VerifyAndWarmInitBegin = 2,
        /// begins warm_init with this config. This does not modify the
        /// forwarding state of the device. However, any subsequent Read /
        /// Write requests must refer to fields in the new config. Returns an
        /// INVALID_ARGUMENT error if the forwarding config is not provided or
        /// if the provided config cannot be realized.
        ///
        /// VERIFY_AND_WARM_INIT_BEGIN_AND_END(Master): Verifies, starts
        VerifyAndWarmInitBeginAndEnd = 3,
        /// warm_init and then initiates warm_init_end on the switch. The
        /// existing forwarding state is reset. Returns an INVALID_ARGUMENT
        /// error if the forwarding config is not provided of if the provided
        /// config cannot be realized.
        ///
        /// WARM_INIT_END(Master): Issues a warm_init_end. If
        WarmInitEnd = 4,
        /// forwarding_pipeline_config contains anything, or if no
        /// WARM_INIT_BEGIN was previously called on the device
        /// with a valid config, then
        /// google.rpc.Code::INVALID_ARGUMENT is sent. The
        /// forwarding state in the target is updated by replaying
        /// the write requests to the target device since the last
        /// config was saved by the client.
        ///
        /// RECONCILE_AND_WARM_INIT_END(Master): Try and reconcile with the
        ReconcileAndWarmInitEnd = 5,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Bind => "BIND",
                Action::Verify => "VERIFY",
                Action::VerifyAndWarmInitBegin => "VERIFY_AND_WARM_INIT_BEGIN",
                Action::VerifyAndWarmInitBeginAndEnd => "VERIFY_AND_WARM_INIT_BEGIN_AND_END",
                Action::WarmInitEnd => "WARM_INIT_END",
                Action::ReconcileAndWarmInitEnd => "RECONCILE_AND_WARM_INIT_END",
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DevInitMode {
        /// This is the default device init mode.
        FastReconfig = 0,
        /// Device incurs a fast-reconfig reset with minimal traffic disruption
        ///
        /// Device incurs a hitless warm init. This incurs even lesser traffic
        Hitless = 1,
    }
    impl DevInitMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DevInitMode::FastReconfig => "FAST_RECONFIG",
                DevInitMode::Hitless => "HITLESS",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetForwardingPipelineConfigResponse {
    #[prost(enumeration = "SetForwardingPipelineConfigResponseType", tag = "1")]
    pub set_forwarding_pipeline_config_response_type: i32,
}
/// This message contains config of a SINGLE program. The reason config is a
/// repeated field in the SetForwardingPipelineConfigRequest is because a
/// device can have multiple programs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingPipelineConfig {
    /// P4 program name
    #[prost(string, tag = "1")]
    pub p4_name: ::prost::alloc::string::String,
    /// BF-RT info json file contents
    #[prost(bytes = "vec", tag = "2")]
    pub bfruntime_info: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub profiles: ::prost::alloc::vec::Vec<forwarding_pipeline_config::Profile>,
}
/// Nested message and enum types in `ForwardingPipelineConfig`.
pub mod forwarding_pipeline_config {
    /// P4 Pipeline Profile
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Profile {
        /// profile name
        #[prost(string, tag = "1")]
        pub profile_name: ::prost::alloc::string::String,
        /// context json file contents
        #[prost(bytes = "vec", tag = "2")]
        pub context: ::prost::alloc::vec::Vec<u8>,
        /// Binary to execute
        #[prost(bytes = "vec", tag = "3")]
        pub binary: ::prost::alloc::vec::Vec<u8>,
        /// Array of pipe_scope.
        #[prost(uint32, repeated, tag = "4")]
        pub pipe_scope: ::prost::alloc::vec::Vec<u32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonP4Config {
    #[prost(bytes = "vec", tag = "1")]
    pub bfruntime_info: ::prost::alloc::vec::Vec<u8>,
}
/// Request to get config of the entire device. Any client can issue this
/// request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetForwardingPipelineConfigRequest {
    #[prost(uint32, tag = "1")]
    pub device_id: u32,
    #[prost(uint32, tag = "2")]
    pub client_id: u32,
}
/// Config of the entire device
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetForwardingPipelineConfigResponse {
    /// P4 info
    #[prost(message, repeated, tag = "1")]
    pub config: ::prost::alloc::vec::Vec<ForwardingPipelineConfig>,
    /// Non-P4 info
    #[prost(message, optional, tag = "2")]
    pub non_p4_config: ::core::option::Option<NonP4Config>,
}
/// Error message used to report a single P4-entity error for a Write RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// gRPC canonical error code (see
    /// github.com/grpc/grpc-go/blob/master/codes/codes.go)
    #[prost(int32, tag = "1")]
    pub canonical_code: i32,
    /// Detailed error message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Target and architecture specific space to which this error belongs.
    /// We encourage using triplet: <target>-<arch>-<vendor>,
    /// e.g."targetX-psa-vendor1" or "targetY-psa-vendor2".
    #[prost(string, tag = "3")]
    pub space: ::prost::alloc::string::String,
    /// Numeric code drawn from target-specific error space above.
    #[prost(int32, tag = "4")]
    pub code: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetForwardingPipelineConfigResponseType {
    /// WARM_INIT_STARTED indicates a successful
    WarmInitStarted = 0,
    // WARM_INIT_BEGIN. This is issued in case of
    // VERIFY_AND_WARM_INIT_BEGIN and
    // VERIFY_AND_WARM_INIT_BEGIN_AND_END
    /// WARM_INIT_FINISHED indicates a successful
    WarmInitFinished = 1,
}
impl SetForwardingPipelineConfigResponseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetForwardingPipelineConfigResponseType::WarmInitStarted => "WARM_INIT_STARTED",
            SetForwardingPipelineConfigResponseType::WarmInitFinished => "WARM_INIT_FINISHED",
        }
    }
}
/// Generated client implementations.
pub mod bf_runtime_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BfRuntimeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BfRuntimeClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BfRuntimeClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BfRuntimeClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BfRuntimeClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Update one or more P4 entities on the target.
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bfrt_proto.BfRuntime/Write");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Read one or more P4 entities from the target.
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bfrt_proto.BfRuntime/Read");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// Sets the P4 fowarding-pipeline config.
        pub async fn set_forwarding_pipeline_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetForwardingPipelineConfigRequest>,
        ) -> Result<tonic::Response<super::SetForwardingPipelineConfigResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bfrt_proto.BfRuntime/SetForwardingPipelineConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the current P4 fowarding-pipeline config.
        pub async fn get_forwarding_pipeline_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetForwardingPipelineConfigRequest>,
        ) -> Result<tonic::Response<super::GetForwardingPipelineConfigResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bfrt_proto.BfRuntime/GetForwardingPipelineConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Represents the bidirectional stream between the controller and the
        /// switch (initiated by the controller).
        pub async fn stream_channel(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamMessageRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamMessageResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/bfrt_proto.BfRuntime/StreamChannel");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod bf_runtime_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with BfRuntimeServer.
    #[async_trait]
    pub trait BfRuntime: Send + Sync + 'static {
        /// Update one or more P4 entities on the target.
        async fn write(
            &self,
            request: tonic::Request<super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status>;
        ///Server streaming response type for the Read method.
        type ReadStream: futures_core::Stream<Item = Result<super::ReadResponse, tonic::Status>>
            + Send
            + 'static;
        /// Read one or more P4 entities from the target.
        async fn read(
            &self,
            request: tonic::Request<super::ReadRequest>,
        ) -> Result<tonic::Response<Self::ReadStream>, tonic::Status>;
        /// Sets the P4 fowarding-pipeline config.
        async fn set_forwarding_pipeline_config(
            &self,
            request: tonic::Request<super::SetForwardingPipelineConfigRequest>,
        ) -> Result<tonic::Response<super::SetForwardingPipelineConfigResponse>, tonic::Status>;
        /// Gets the current P4 fowarding-pipeline config.
        async fn get_forwarding_pipeline_config(
            &self,
            request: tonic::Request<super::GetForwardingPipelineConfigRequest>,
        ) -> Result<tonic::Response<super::GetForwardingPipelineConfigResponse>, tonic::Status>;
        ///Server streaming response type for the StreamChannel method.
        type StreamChannelStream: futures_core::Stream<Item = Result<super::StreamMessageResponse, tonic::Status>>
            + Send
            + 'static;
        /// Represents the bidirectional stream between the controller and the
        /// switch (initiated by the controller).
        async fn stream_channel(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamMessageRequest>>,
        ) -> Result<tonic::Response<Self::StreamChannelStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BfRuntimeServer<T: BfRuntime> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BfRuntime> BfRuntimeServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BfRuntimeServer<T>
    where
        T: BfRuntime,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/bfrt_proto.BfRuntime/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: BfRuntime>(pub Arc<T>);
                    impl<T: BfRuntime> tonic::server::UnaryService<super::WriteRequest> for WriteSvc<T> {
                        type Response = super::WriteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).write(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bfrt_proto.BfRuntime/Read" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSvc<T: BfRuntime>(pub Arc<T>);
                    impl<T: BfRuntime> tonic::server::ServerStreamingService<super::ReadRequest> for ReadSvc<T> {
                        type Response = super::ReadResponse;
                        type ResponseStream = T::ReadStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bfrt_proto.BfRuntime/SetForwardingPipelineConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetForwardingPipelineConfigSvc<T: BfRuntime>(pub Arc<T>);
                    impl<T: BfRuntime>
                        tonic::server::UnaryService<super::SetForwardingPipelineConfigRequest>
                        for SetForwardingPipelineConfigSvc<T>
                    {
                        type Response = super::SetForwardingPipelineConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetForwardingPipelineConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_forwarding_pipeline_config(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetForwardingPipelineConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bfrt_proto.BfRuntime/GetForwardingPipelineConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetForwardingPipelineConfigSvc<T: BfRuntime>(pub Arc<T>);
                    impl<T: BfRuntime>
                        tonic::server::UnaryService<super::GetForwardingPipelineConfigRequest>
                        for GetForwardingPipelineConfigSvc<T>
                    {
                        type Response = super::GetForwardingPipelineConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetForwardingPipelineConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_forwarding_pipeline_config(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetForwardingPipelineConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bfrt_proto.BfRuntime/StreamChannel" => {
                    #[allow(non_camel_case_types)]
                    struct StreamChannelSvc<T: BfRuntime>(pub Arc<T>);
                    impl<T: BfRuntime> tonic::server::StreamingService<super::StreamMessageRequest>
                        for StreamChannelSvc<T>
                    {
                        type Response = super::StreamMessageResponse;
                        type ResponseStream = T::StreamChannelStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::StreamMessageRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_channel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamChannelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BfRuntime> Clone for BfRuntimeServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BfRuntime> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BfRuntime> tonic::server::NamedService for BfRuntimeServer<T> {
        const NAME: &'static str = "bfrt_proto.BfRuntime";
    }
}
