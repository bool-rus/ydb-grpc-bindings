pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
pub mod ydb {
    pub mod auth {
        pub mod v1 {
            include!("ydb.auth.v1.rs");
        }
        include!("ydb.auth.rs");
    }
    pub mod cms {
        pub mod v1 {
            include!("ydb.cms.v1.rs");
        }
        include!("ydb.cms.rs");
    }
    pub mod coordination {
        pub mod v1 {
            include!("ydb.coordination.v1.rs");
        }
        include!("ydb.coordination.rs");
    }
    pub mod discovery {
        pub mod v1 {
            include!("ydb.discovery.v1.rs");
        }
        include!("ydb.discovery.rs");
    }
    pub mod export {
        pub mod v1 {
            include!("ydb.export.v1.rs");
        }
        include!("ydb.export.rs");
    }
    pub mod formats {
        include!("ydb.formats.rs");
    }
    pub mod import {
        pub mod v1 {
            include!("ydb.import.v1.rs");
        }
        include!("ydb.import.rs");
    }
    pub mod issue {
        include!("ydb.issue.rs");
    }
    pub mod monitoring {
        pub mod v1 {
            include!("ydb.monitoring.v1.rs");
        }
        include!("ydb.monitoring.rs");
    }
    pub mod operation {
        pub mod v1 {
            include!("ydb.operation.v1.rs");
        }
    }
    pub mod operations {
        include!("ydb.operations.rs");
    }
    pub mod rate_limiter {
        pub mod v1 {
            include!("ydb.rate_limiter.v1.rs");
        }
        include!("ydb.rate_limiter.rs");
    }
    pub mod scheme {
        pub mod v1 {
            include!("ydb.scheme.v1.rs");
        }
        include!("ydb.scheme.rs");
    }
    pub mod scripting {
        pub mod v1 {
            include!("ydb.scripting.v1.rs");
        }
        include!("ydb.scripting.rs");
    }
    pub mod table {
        pub mod v1 {
            include!("ydb.table.v1.rs");
        }
        include!("ydb.table.rs");
    }
    pub mod table_stats {
        include!("ydb.table_stats.rs");
    }
    pub mod topic {
        pub mod v1 {
            include!("ydb.topic.v1.rs");
        }
        include!("ydb.topic.rs");
    }
    include!("ydb.rs");
}
