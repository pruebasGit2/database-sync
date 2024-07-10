use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  tonic_build::compile_protos("../protos/database/v1/database.proto")?;
  tauri_build::build();
  Ok(())
}
