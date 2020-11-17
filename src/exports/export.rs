use crate::{
  core::suiter::Suite,
  exports::json::JSON,
};


use std::{
  fs,
  io::Write,
  path::Path,
};


pub trait Exporter {
  fn new (data: Suite) -> Self;
  fn parse (self) -> Self;
  fn path (self, pathname: String) -> Self;
}


#[derive(Clone, Copy, Debug)]
pub enum ExportMode {
  JSON,
}


#[derive(Debug)]
pub struct ExportConfig {
  pub data: Suite,
  pub filename: &'static str,
  pub pathname: &'static str,
}


impl Clone for ExportConfig {
  fn clone (&self) -> Self {
    ExportConfig {
      data: self.data.clone(),
      filename: self.filename.clone(),
      pathname: self.pathname.clone(),
    }
  }
}


#[derive(Debug)]
pub struct Export {
  pub config: ExportConfig,
  pub exports: Vec<String>,
  pub mode: ExportMode,
}


impl Export {
  pub fn new (config: ExportConfig) -> Self {
    Export {
      config,
      exports: vec![],
      mode: ExportMode::JSON,
    }
  }

  pub fn mode (mut self, mode: ExportMode) -> Self {
    self.mode = mode;
    self
  }

  // TODO: refacto!
  pub fn to_json (mut self) {
    println!("\ncreating json...");

    let config = self.config.clone();
    let path = format!("{}/{}", config.pathname, config.filename);
    let json = JSON::new(config.data).path(path).parse();

    self.exports.push(json.output.clone());

    if let Some(parent) = Path::new(&json.pathname).parent() {
      if !parent.exists() {
        fs::create_dir_all(parent)
        .unwrap_or_else(|_| panic!("Could not create {:?}", json.pathname));
      }
    }

    let mut file = fs::File::create(&json.pathname)
    .expect("Could not create output file");

    file.write_all(self.exports[ExportMode::JSON as usize].as_bytes())
    .expect("Could not output to file");

    println!("created json in {}", &json.pathname);
  }
}
