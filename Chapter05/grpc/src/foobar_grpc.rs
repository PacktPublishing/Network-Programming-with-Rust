// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait FooBarService {
    fn record_cab_location(&self, o: ::grpc::RequestOptions, p: super::foobar::CabLocationRequest) -> ::grpc::SingleResponse<super::foobar::CabLocationResponse>;

    fn get_cabs(&self, o: ::grpc::RequestOptions, p: super::foobar::GetCabRequest) -> ::grpc::SingleResponse<super::foobar::GetCabResponse>;
}

// client

pub struct FooBarServiceClient {
    grpc_client: ::grpc::Client,
    method_record_cab_location: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::foobar::CabLocationRequest, super::foobar::CabLocationResponse>>,
    method_get_cabs: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::foobar::GetCabRequest, super::foobar::GetCabResponse>>,
}

impl FooBarServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        FooBarServiceClient {
            grpc_client: grpc_client,
            method_record_cab_location: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/foobar.FooBarService/record_cab_location".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_get_cabs: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/foobar.FooBarService/get_cabs".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            FooBarServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            FooBarServiceClient::with_client(c)
        })
    }
}

impl FooBarService for FooBarServiceClient {
    fn record_cab_location(&self, o: ::grpc::RequestOptions, p: super::foobar::CabLocationRequest) -> ::grpc::SingleResponse<super::foobar::CabLocationResponse> {
        self.grpc_client.call_unary(o, p, self.method_record_cab_location.clone())
    }

    fn get_cabs(&self, o: ::grpc::RequestOptions, p: super::foobar::GetCabRequest) -> ::grpc::SingleResponse<super::foobar::GetCabResponse> {
        self.grpc_client.call_unary(o, p, self.method_get_cabs.clone())
    }
}

// server

pub struct FooBarServiceServer;


impl FooBarServiceServer {
    pub fn new_service_def<H : FooBarService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/foobar.FooBarService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/foobar.FooBarService/record_cab_location".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.record_cab_location(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/foobar.FooBarService/get_cabs".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_cabs(o, p))
                    },
                ),
            ],
        )
    }
}
