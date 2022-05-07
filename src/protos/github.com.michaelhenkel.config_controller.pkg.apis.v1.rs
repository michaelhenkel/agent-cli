#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    #[prost(string, tag="1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub namespace: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(enumeration="action::Action", tag="1")]
    pub action: i32,
}
/// Nested message and enum types in `Action`.
pub mod action {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        Add = 0,
        Del = 1,
        Retry = 2,
        Get = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyAction {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<Key>,
    #[prost(enumeration="key_action::Action", tag="2")]
    pub action: i32,
}
/// Nested message and enum types in `KeyAction`.
pub mod key_action {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        Add = 0,
        Del = 1,
        Retry = 2,
        Get = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(oneof="resource::Resource", tags="1, 2, 3, 4, 5, 6, 7")]
    pub resource: ::core::option::Option<resource::Resource>,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(message, tag="1")]
        VirtualNetwork(super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork),
        #[prost(message, tag="2")]
        VirtualMachineInterface(super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualMachineInterface),
        #[prost(message, tag="3")]
        VirtualRouter(super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualRouter),
        #[prost(message, tag="4")]
        VirtualMachine(super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualMachine),
        #[prost(message, tag="5")]
        InstanceIp(super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::InstanceIp),
        #[prost(message, tag="6")]
        RoutingInstance(super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::RoutingInstance),
        #[prost(message, tag="7")]
        Namespace(super::super::super::super::super::super::super::super::k8s::io::api::core::v1::Namespace),
    }
}
/// Generated client implementations.
pub mod config_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ConfigControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConfigControllerClient<tonic::transport::Channel> {
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
    impl<T> ConfigControllerClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ConfigControllerClient<InterceptedService<T, F>>
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
            ConfigControllerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn subscribe_list_watch(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::KeyAction>>,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/SubscribeListWatch",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_virtual_network(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualNetwork",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_virtual_machine_interface(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualMachineInterface,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualMachineInterface",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_virtual_machine(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualMachine,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualMachine",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_virtual_router(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualRouter,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualRouter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_routing_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::RoutingInstance,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetRoutingInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_instance_ip(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::super::super::super::super::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::InstanceIp,
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetInstanceIP",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod cli_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CliClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CliClient<tonic::transport::Channel> {
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
    impl<T> CliClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CliClient<InterceptedService<T, F>>
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
            CliClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<tonic::Response<super::Resource>, tonic::Status> {
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
                "/github.com.michaelhenkel.config_controller.pkg.apis.v1.Cli/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
