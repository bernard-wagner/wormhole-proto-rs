
fn main() {
    protobuf_codegen::Codegen::new()
    .include("protos")
    .includes(&["protos"])
    .inputs(&["protos/gossip/v1/gossip.proto", "protos/publicrpc/v1/publicrpc.proto", "protos/spy/v1/spy.proto"])
    .out_dir("src/protos")
    .run().unwrap();


}

