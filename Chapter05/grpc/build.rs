extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &[],
        input: &["foobar.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");
}
