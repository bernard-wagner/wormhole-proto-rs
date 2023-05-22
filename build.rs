
fn main() {
    protoc_rust_grpc::Codegen::new()
    .includes(&["protos"])
    .inputs(&["protos/gossip/v1/gossip.proto", "protos/publicrpc/v1/publicrpc.proto", "protos/spy/v1/spy.proto"])
    .out_dir("src")
    .rust_protobuf(true)
    .run().unwrap();


}

