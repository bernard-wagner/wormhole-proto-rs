// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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


// server interface

pub trait SpyRPCService {
    fn subscribe_signed_vaa(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::spy::SubscribeSignedVAARequest>, resp: ::grpc::ServerResponseSink<super::spy::SubscribeSignedVAAResponse>) -> ::grpc::Result<()>;

    fn subscribe_signed_vaa_by_type(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::spy::SubscribeSignedVAAByTypeRequest>, resp: ::grpc::ServerResponseSink<super::spy::SubscribeSignedVAAByTypeResponse>) -> ::grpc::Result<()>;
}

// client

pub struct SpyRPCServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for SpyRPCServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        SpyRPCServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl SpyRPCServiceClient {
    pub fn subscribe_signed_vaa(&self, o: ::grpc::RequestOptions, req: super::spy::SubscribeSignedVAARequest) -> ::grpc::StreamingResponse<super::spy::SubscribeSignedVAAResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/spy.v1.SpyRPCService/SubscribeSignedVAA"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }

    pub fn subscribe_signed_vaa_by_type(&self, o: ::grpc::RequestOptions, req: super::spy::SubscribeSignedVAAByTypeRequest) -> ::grpc::StreamingResponse<super::spy::SubscribeSignedVAAByTypeResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/spy.v1.SpyRPCService/SubscribeSignedVAAByType"),
            streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_server_streaming(o, req, descriptor)
    }
}

// server

pub struct SpyRPCServiceServer;


impl SpyRPCServiceServer {
    pub fn new_service_def<H : SpyRPCService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/spy.v1.SpyRPCService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/spy.v1.SpyRPCService/SubscribeSignedVAA"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).subscribe_signed_vaa(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/spy.v1.SpyRPCService/SubscribeSignedVAAByType"),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |ctx, req, resp| (*handler_copy).subscribe_signed_vaa_by_type(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
