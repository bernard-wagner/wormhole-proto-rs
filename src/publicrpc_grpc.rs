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

pub trait PublicRPCService {
    fn get_last_heartbeats(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GetLastHeartbeatsRequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GetLastHeartbeatsResponse>) -> ::grpc::Result<()>;

    fn get_signed_vaa(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GetSignedVAARequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GetSignedVAAResponse>) -> ::grpc::Result<()>;

    fn get_signed_batch_vaa(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GetSignedBatchVAARequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GetSignedBatchVAAResponse>) -> ::grpc::Result<()>;

    fn get_current_guardian_set(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GetCurrentGuardianSetRequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GetCurrentGuardianSetResponse>) -> ::grpc::Result<()>;

    fn governor_get_available_notional_by_chain(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GovernorGetAvailableNotionalByChainRequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GovernorGetAvailableNotionalByChainResponse>) -> ::grpc::Result<()>;

    fn governor_get_enqueued_va_as(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GovernorGetEnqueuedVAAsRequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GovernorGetEnqueuedVAAsResponse>) -> ::grpc::Result<()>;

    fn governor_is_vaa_enqueued(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GovernorIsVAAEnqueuedRequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GovernorIsVAAEnqueuedResponse>) -> ::grpc::Result<()>;

    fn governor_get_token_list(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::publicrpc::GovernorGetTokenListRequest>, resp: ::grpc::ServerResponseUnarySink<super::publicrpc::GovernorGetTokenListResponse>) -> ::grpc::Result<()>;
}

// client

pub struct PublicRPCServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for PublicRPCServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        PublicRPCServiceClient {
            grpc_client: grpc_client,
        }
    }
}

impl PublicRPCServiceClient {
    pub fn get_last_heartbeats(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GetLastHeartbeatsRequest) -> ::grpc::SingleResponse<super::publicrpc::GetLastHeartbeatsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetLastHeartbeats"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_signed_vaa(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GetSignedVAARequest) -> ::grpc::SingleResponse<super::publicrpc::GetSignedVAAResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetSignedVAA"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_signed_batch_vaa(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GetSignedBatchVAARequest) -> ::grpc::SingleResponse<super::publicrpc::GetSignedBatchVAAResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetSignedBatchVAA"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn get_current_guardian_set(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GetCurrentGuardianSetRequest) -> ::grpc::SingleResponse<super::publicrpc::GetCurrentGuardianSetResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetCurrentGuardianSet"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn governor_get_available_notional_by_chain(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GovernorGetAvailableNotionalByChainRequest) -> ::grpc::SingleResponse<super::publicrpc::GovernorGetAvailableNotionalByChainResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorGetAvailableNotionalByChain"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn governor_get_enqueued_va_as(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GovernorGetEnqueuedVAAsRequest) -> ::grpc::SingleResponse<super::publicrpc::GovernorGetEnqueuedVAAsResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorGetEnqueuedVAAs"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn governor_is_vaa_enqueued(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GovernorIsVAAEnqueuedRequest) -> ::grpc::SingleResponse<super::publicrpc::GovernorIsVAAEnqueuedResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorIsVAAEnqueued"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }

    pub fn governor_get_token_list(&self, o: ::grpc::RequestOptions, req: super::publicrpc::GovernorGetTokenListRequest) -> ::grpc::SingleResponse<super::publicrpc::GovernorGetTokenListResponse> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorGetTokenList"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct PublicRPCServiceServer;


impl PublicRPCServiceServer {
    pub fn new_service_def<H : PublicRPCService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/publicrpc.v1.PublicRPCService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetLastHeartbeats"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_last_heartbeats(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetSignedVAA"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_signed_vaa(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetSignedBatchVAA"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_signed_batch_vaa(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GetCurrentGuardianSet"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_current_guardian_set(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorGetAvailableNotionalByChain"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).governor_get_available_notional_by_chain(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorGetEnqueuedVAAs"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).governor_get_enqueued_va_as(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorIsVAAEnqueued"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).governor_is_vaa_enqueued(ctx, req, resp))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/publicrpc.v1.PublicRPCService/GovernorGetTokenList"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).governor_get_token_list(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
