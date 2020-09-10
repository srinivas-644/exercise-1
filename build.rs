extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/proto",
        input: &["src/proto/foo.proto"],
        includes: &["src/proto"],
        customize: Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}
