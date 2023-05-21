/// Generated client implementations.
pub mod table_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TableServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TableServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TableServiceClient<T>
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
        ) -> TableServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            TableServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Create new session. Implicit session creation is forbidden,
        /// so user must create new session before execute any query,
        /// otherwise BAD_SESSION status will be returned.
        /// Simultaneous execution of requests are forbiden.
        /// Sessions are volatile, can be invalidated by server, for example in case
        /// of fatal errors. All requests with this session will fail with BAD_SESSION status.
        /// So, client must be able to handle BAD_SESSION status.
        pub async fn create_session(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CreateSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::CreateSessionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/CreateSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "CreateSession"));
            self.inner.unary(req, path, codec).await
        }
        /// Ends a session, releasing server resources associated with it.
        pub async fn delete_session(
            &mut self,
            request: impl tonic::IntoRequest<super::super::DeleteSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::DeleteSessionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/DeleteSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "DeleteSession"));
            self.inner.unary(req, path, codec).await
        }
        /// Idle sessions can be kept alive by calling KeepAlive periodically.
        pub async fn keep_alive(
            &mut self,
            request: impl tonic::IntoRequest<super::super::KeepAliveRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::KeepAliveResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/KeepAlive",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "KeepAlive"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates new table.
        pub async fn create_table(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CreateTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::CreateTableResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/CreateTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "CreateTable"));
            self.inner.unary(req, path, codec).await
        }
        /// Drop table.
        pub async fn drop_table(
            &mut self,
            request: impl tonic::IntoRequest<super::super::DropTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::DropTableResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/DropTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "DropTable"));
            self.inner.unary(req, path, codec).await
        }
        /// Modifies schema of given table.
        pub async fn alter_table(
            &mut self,
            request: impl tonic::IntoRequest<super::super::AlterTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::AlterTableResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/AlterTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "AlterTable"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates copy of given table.
        pub async fn copy_table(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CopyTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::CopyTableResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/CopyTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "CopyTable"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates consistent copy of given tables.
        pub async fn copy_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CopyTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::CopyTablesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/CopyTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "CopyTables"));
            self.inner.unary(req, path, codec).await
        }
        /// Creates consistent move of given tables.
        pub async fn rename_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::super::RenameTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::RenameTablesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/RenameTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "RenameTables"));
            self.inner.unary(req, path, codec).await
        }
        /// Returns information about given table (metadata).
        pub async fn describe_table(
            &mut self,
            request: impl tonic::IntoRequest<super::super::DescribeTableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::DescribeTableResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/DescribeTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "DescribeTable"));
            self.inner.unary(req, path, codec).await
        }
        /// Explains data query.
        /// SessionId of previously created session must be provided.
        pub async fn explain_data_query(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExplainDataQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::ExplainDataQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/ExplainDataQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "ExplainDataQuery"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Prepares data query, returns query id.
        /// SessionId of previously created session must be provided.
        pub async fn prepare_data_query(
            &mut self,
            request: impl tonic::IntoRequest<super::super::PrepareDataQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::PrepareDataQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/PrepareDataQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "PrepareDataQuery"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes data query.
        /// SessionId of previously created session must be provided.
        pub async fn execute_data_query(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteDataQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::ExecuteDataQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/ExecuteDataQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "ExecuteDataQuery"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Executes scheme query.
        /// SessionId of previously created session must be provided.
        pub async fn execute_scheme_query(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteSchemeQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::ExecuteSchemeQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/ExecuteSchemeQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "ExecuteSchemeQuery"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Begins new transaction.
        pub async fn begin_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::BeginTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::BeginTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/BeginTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "BeginTransaction"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Commits specified active transaction.
        pub async fn commit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CommitTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::CommitTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/CommitTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "CommitTransaction"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Performs a rollback of the specified active transaction.
        pub async fn rollback_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::RollbackTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::RollbackTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/RollbackTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "RollbackTransaction"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Describe supported table options.
        pub async fn describe_table_options(
            &mut self,
            request: impl tonic::IntoRequest<super::super::DescribeTableOptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::DescribeTableOptionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/DescribeTableOptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("Ydb.Table.V1.TableService", "DescribeTableOptions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Streaming read table
        pub async fn stream_read_table(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ReadTableRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::super::ReadTableResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/StreamReadTable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "StreamReadTable"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Upserts a batch of rows non-transactionally.
        /// Returns success only when all rows were successfully upserted. In case of an error some rows might
        /// be upserted and some might not.
        pub async fn bulk_upsert(
            &mut self,
            request: impl tonic::IntoRequest<super::super::BulkUpsertRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::BulkUpsertResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/BulkUpsert",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Ydb.Table.V1.TableService", "BulkUpsert"));
            self.inner.unary(req, path, codec).await
        }
        /// Executes scan query with streaming result.
        pub async fn stream_execute_scan_query(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteScanQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::ExecuteScanQueryPartialResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Table.V1.TableService/StreamExecuteScanQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "Ydb.Table.V1.TableService",
                        "StreamExecuteScanQuery",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
