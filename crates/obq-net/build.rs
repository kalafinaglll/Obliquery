fn main() {
    tonic_build::compile_protos("proto/rss3.proto").unwrap();
}