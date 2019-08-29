extern crate protoc_grpcio;

use std::fs;
use std::path::Path;

fn main() {
    let proto_root = "src/protos";
    let proto_out = "src/protos_gen";
    if !Path::new(proto_out).exists() {
        fs::create_dir(proto_out);
    }
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &[
            "enum.proto",
            "raft.proto",
            "rpc.proto",
            "log.proto",
            "diner.proto",
        ],
        &[proto_root],
        &proto_out,
        None,
    )
        .expect("Failed to compile gRPC definitions!");
}
