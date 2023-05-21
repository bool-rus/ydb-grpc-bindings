/// / Common
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportProgress {}
/// Nested message and enum types in `ExportProgress`.
pub mod export_progress {
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
    pub enum Progress {
        Unspecified = 0,
        Preparing = 1,
        TransferData = 2,
        Done = 3,
        Cancellation = 4,
        Cancelled = 5,
    }
    impl Progress {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Progress::Unspecified => "PROGRESS_UNSPECIFIED",
                Progress::Preparing => "PROGRESS_PREPARING",
                Progress::TransferData => "PROGRESS_TRANSFER_DATA",
                Progress::Done => "PROGRESS_DONE",
                Progress::Cancellation => "PROGRESS_CANCELLATION",
                Progress::Cancelled => "PROGRESS_CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROGRESS_UNSPECIFIED" => Some(Self::Unspecified),
                "PROGRESS_PREPARING" => Some(Self::Preparing),
                "PROGRESS_TRANSFER_DATA" => Some(Self::TransferData),
                "PROGRESS_DONE" => Some(Self::Done),
                "PROGRESS_CANCELLATION" => Some(Self::Cancellation),
                "PROGRESS_CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportItemProgress {
    #[prost(uint32, tag = "1")]
    pub parts_total: u32,
    #[prost(uint32, tag = "2")]
    pub parts_completed: u32,
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
}
/// / YT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToYtSettings {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub items: ::prost::alloc::vec::Vec<export_to_yt_settings::Item>,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub number_of_retries: u32,
    #[prost(bool, tag = "7")]
    pub use_type_v3: bool,
}
/// Nested message and enum types in `ExportToYtSettings`.
pub mod export_to_yt_settings {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// Database path to a table to be exported
        #[prost(string, tag = "1")]
        pub source_path: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub destination_path: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToYtResult {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToYtMetadata {
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<ExportToYtSettings>,
    #[prost(enumeration = "export_progress::Progress", tag = "2")]
    pub progress: i32,
    #[prost(message, repeated, tag = "3")]
    pub items_progress: ::prost::alloc::vec::Vec<ExportItemProgress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToYtRequest {
    #[prost(message, optional, tag = "1")]
    pub operation_params: ::core::option::Option<super::operations::OperationParams>,
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<ExportToYtSettings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToYtResponse {
    /// operation.result = ExportToYtResult
    /// operation.metadata = ExportToYtMetadata
    #[prost(message, optional, tag = "1")]
    pub operation: ::core::option::Option<super::operations::Operation>,
}
/// / S3
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToS3Settings {
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// HTTPS if not specified
    #[prost(enumeration = "export_to_s3_settings::Scheme", tag = "2")]
    pub scheme: i32,
    #[prost(string, tag = "3")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub secret_key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<export_to_s3_settings::Item>,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "8")]
    pub number_of_retries: u32,
    #[prost(enumeration = "export_to_s3_settings::StorageClass", tag = "9")]
    pub storage_class: i32,
    /// Codec used to compress data. Codecs are available:
    /// - zstd.
    /// - zstd-N, where N is compression level, e.g. zstd-3.
    #[prost(string, tag = "10")]
    pub compression: ::prost::alloc::string::String,
    /// Region to use in requests
    #[prost(string, tag = "11")]
    pub region: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExportToS3Settings`.
pub mod export_to_s3_settings {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// Database path to a table to be exported
        #[prost(string, tag = "1")]
        pub source_path: ::prost::alloc::string::String,
        /// Tables are exported to one or more S3 objects.
        /// The object name begins with 'destination_prefix'.
        /// This prefix will be followed by '/data_PartNumber', where 'PartNumber'
        /// represents the index of the part, starting at zero.
        #[prost(string, tag = "2")]
        pub destination_prefix: ::prost::alloc::string::String,
    }
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
    pub enum Scheme {
        Unspecified = 0,
        Http = 1,
        Https = 2,
    }
    impl Scheme {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scheme::Unspecified => "UNSPECIFIED",
                Scheme::Http => "HTTP",
                Scheme::Https => "HTTPS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "HTTP" => Some(Self::Http),
                "HTTPS" => Some(Self::Https),
                _ => None,
            }
        }
    }
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
    pub enum StorageClass {
        Unspecified = 0,
        Standard = 1,
        ReducedRedundancy = 2,
        StandardIa = 3,
        OnezoneIa = 4,
        IntelligentTiering = 5,
        Glacier = 6,
        DeepArchive = 7,
        Outposts = 8,
    }
    impl StorageClass {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StorageClass::Unspecified => "STORAGE_CLASS_UNSPECIFIED",
                StorageClass::Standard => "STANDARD",
                StorageClass::ReducedRedundancy => "REDUCED_REDUNDANCY",
                StorageClass::StandardIa => "STANDARD_IA",
                StorageClass::OnezoneIa => "ONEZONE_IA",
                StorageClass::IntelligentTiering => "INTELLIGENT_TIERING",
                StorageClass::Glacier => "GLACIER",
                StorageClass::DeepArchive => "DEEP_ARCHIVE",
                StorageClass::Outposts => "OUTPOSTS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STORAGE_CLASS_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD" => Some(Self::Standard),
                "REDUCED_REDUNDANCY" => Some(Self::ReducedRedundancy),
                "STANDARD_IA" => Some(Self::StandardIa),
                "ONEZONE_IA" => Some(Self::OnezoneIa),
                "INTELLIGENT_TIERING" => Some(Self::IntelligentTiering),
                "GLACIER" => Some(Self::Glacier),
                "DEEP_ARCHIVE" => Some(Self::DeepArchive),
                "OUTPOSTS" => Some(Self::Outposts),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToS3Result {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToS3Metadata {
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<ExportToS3Settings>,
    #[prost(enumeration = "export_progress::Progress", tag = "2")]
    pub progress: i32,
    #[prost(message, repeated, tag = "3")]
    pub items_progress: ::prost::alloc::vec::Vec<ExportItemProgress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToS3Request {
    #[prost(message, optional, tag = "1")]
    pub operation_params: ::core::option::Option<super::operations::OperationParams>,
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<ExportToS3Settings>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportToS3Response {
    /// operation.result = ExportToS3Result
    /// operation.metadata = ExportToS3Metadata
    #[prost(message, optional, tag = "1")]
    pub operation: ::core::option::Option<super::operations::Operation>,
}
