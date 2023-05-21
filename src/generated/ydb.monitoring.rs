#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusFlag {}
/// Nested message and enum types in `StatusFlag`.
pub mod status_flag {
    /// Describes the general state of a component.
    /// From GREEN to RED, where GREEN is good, and RED is bad.
    /// GREY means that the corresponding status is unknown.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        Grey = 1,
        Green = 2,
        Blue = 3,
        Yellow = 4,
        Orange = 5,
        Red = 6,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "UNSPECIFIED",
                Status::Grey => "GREY",
                Status::Green => "GREEN",
                Status::Blue => "BLUE",
                Status::Yellow => "YELLOW",
                Status::Orange => "ORANGE",
                Status::Red => "RED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "GREY" => Some(Self::Grey),
                "GREEN" => Some(Self::Green),
                "BLUE" => Some(Self::Blue),
                "YELLOW" => Some(Self::Yellow),
                "ORANGE" => Some(Self::Orange),
                "RED" => Some(Self::Red),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfCheckRequest {
    /// basic operation params, including timeout
    #[prost(message, optional, tag = "1")]
    pub operation_params: ::core::option::Option<super::operations::OperationParams>,
    /// return detailed info about components checked with their statuses
    #[prost(bool, tag = "2")]
    pub return_verbose_status: bool,
    /// minimum status of issues to return
    #[prost(enumeration = "status_flag::Status", tag = "3")]
    pub minimum_status: i32,
    /// maximum level of issues to return
    #[prost(uint32, tag = "4")]
    pub maximum_level: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfCheckResponse {
    /// After successfull completion must contain SelfCheckResult.
    #[prost(message, optional, tag = "1")]
    pub operation: ::core::option::Option<super::operations::Operation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeCheckRequest {
    /// basic operation params, including timeout
    #[prost(message, optional, tag = "1")]
    pub operation_params: ::core::option::Option<super::operations::OperationParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeCheckResponse {
    /// After successfull completion must contain SelfCheckResult.
    #[prost(message, optional, tag = "1")]
    pub operation: ::core::option::Option<super::operations::Operation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfCheck {}
/// Nested message and enum types in `SelfCheck`.
pub mod self_check {
    /// Describes the result of self-check performed.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Result {
        Unspecified = 0,
        Good = 1,
        Degraded = 2,
        MaintenanceRequired = 3,
        Emergency = 4,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unspecified => "UNSPECIFIED",
                Result::Good => "GOOD",
                Result::Degraded => "DEGRADED",
                Result::MaintenanceRequired => "MAINTENANCE_REQUIRED",
                Result::Emergency => "EMERGENCY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "GOOD" => Some(Self::Good),
                "DEGRADED" => Some(Self::Degraded),
                "MAINTENANCE_REQUIRED" => Some(Self::MaintenanceRequired),
                "EMERGENCY" => Some(Self::Emergency),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePDiskStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub overall: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVDiskStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub overall: i32,
    #[prost(enumeration = "status_flag::Status", tag = "3")]
    pub vdisk_status: i32,
    #[prost(message, optional, tag = "4")]
    pub pdisk: ::core::option::Option<StoragePDiskStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageGroupStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub overall: i32,
    #[prost(message, repeated, tag = "3")]
    pub vdisks: ::prost::alloc::vec::Vec<StorageVDiskStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoragePoolStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub overall: i32,
    #[prost(message, repeated, tag = "3")]
    pub groups: ::prost::alloc::vec::Vec<StorageGroupStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageStatus {
    #[prost(enumeration = "status_flag::Status", tag = "1")]
    pub overall: i32,
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<StoragePoolStatus>,
}
/// Describes the state of a tablet group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeTabletStatus {
    #[prost(enumeration = "status_flag::Status", tag = "1")]
    pub overall: i32,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub count: u32,
    #[prost(string, repeated, tag = "5")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreadPoolStatus {
    #[prost(enumeration = "status_flag::Status", tag = "1")]
    pub overall: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(float, tag = "3")]
    pub usage: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadAverageStatus {
    #[prost(enumeration = "status_flag::Status", tag = "1")]
    pub overall: i32,
    #[prost(float, tag = "2")]
    pub load: f32,
    #[prost(uint32, tag = "3")]
    pub cores: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeNodeStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub overall: i32,
    #[prost(message, repeated, tag = "3")]
    pub tablets: ::prost::alloc::vec::Vec<ComputeTabletStatus>,
    #[prost(message, repeated, tag = "4")]
    pub pools: ::prost::alloc::vec::Vec<ThreadPoolStatus>,
    #[prost(message, optional, tag = "5")]
    pub load: ::core::option::Option<LoadAverageStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeStatus {
    #[prost(enumeration = "status_flag::Status", tag = "1")]
    pub overall: i32,
    #[prost(message, repeated, tag = "2")]
    pub nodes: ::prost::alloc::vec::Vec<ComputeNodeStatus>,
    #[prost(message, repeated, tag = "3")]
    pub tablets: ::prost::alloc::vec::Vec<ComputeTabletStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationNode {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationStoragePDisk {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationStorageVDisk {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub pdisk: ::prost::alloc::vec::Vec<LocationStoragePDisk>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationStorageGroup {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub vdisk: ::core::option::Option<LocationStorageVDisk>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationStoragePool {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<LocationStorageGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationStorage {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<LocationNode>,
    #[prost(message, optional, tag = "2")]
    pub pool: ::core::option::Option<LocationStoragePool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationComputePool {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationComputeTablet {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "3")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationCompute {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<LocationNode>,
    #[prost(message, optional, tag = "2")]
    pub pool: ::core::option::Option<LocationComputePool>,
    #[prost(message, optional, tag = "3")]
    pub tablet: ::core::option::Option<LocationComputeTablet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationDatabase {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(message, optional, tag = "1")]
    pub storage: ::core::option::Option<LocationStorage>,
    #[prost(message, optional, tag = "2")]
    pub compute: ::core::option::Option<LocationCompute>,
    #[prost(message, optional, tag = "3")]
    pub database: ::core::option::Option<LocationDatabase>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueLog {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub status: i32,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub location: ::core::option::Option<Location>,
    #[prost(string, repeated, tag = "5")]
    pub reason: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub level: u32,
    #[prost(uint32, tag = "8")]
    pub listed: u32,
    #[prost(uint32, tag = "9")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseStatus {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "status_flag::Status", tag = "2")]
    pub overall: i32,
    #[prost(message, optional, tag = "3")]
    pub storage: ::core::option::Option<StorageStatus>,
    #[prost(message, optional, tag = "4")]
    pub compute: ::core::option::Option<ComputeStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfCheckResult {
    #[prost(enumeration = "self_check::Result", tag = "1")]
    pub self_check_result: i32,
    #[prost(message, repeated, tag = "2")]
    pub issue_log: ::prost::alloc::vec::Vec<IssueLog>,
    #[prost(message, repeated, tag = "3")]
    pub database_status: ::prost::alloc::vec::Vec<DatabaseStatus>,
}
