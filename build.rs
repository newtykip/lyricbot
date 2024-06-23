use prost_build::Config;
use std::io::Result;

fn main() -> Result<()> {
    let mut builder = Config::new();

    // allow the optional keyword in .proto
    builder.protoc_arg("--experimental_allow_proto3_optional");
    builder.compile_protos(&["src/profile.proto"], &["src/"])?;

    Ok(())
}
