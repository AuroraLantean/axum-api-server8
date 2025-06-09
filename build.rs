use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
  //OUT_DIR is the folder in which all output and intermediate artifacts should be places
  let descriptor_path =
    PathBuf::from(env::var("OUT_DIR").unwrap()).join("calculator_descriptor.bin");

  tonic_build::configure()
    .file_descriptor_set_path(&descriptor_path)
    .compile_protos(&["proto/calculator.proto"], &["proto/"])?;

  tonic_build::compile_protos("proto/calculator.proto")?;
  Ok(())
}
