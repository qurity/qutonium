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
  fn name (self, filename: &'static str) -> Self;
  fn parse (self) -> String;
  fn path (self, pathname: &'static str) -> Self;
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
    let config = self.config.clone();
    let path = format!("{}/{}", config.pathname, config.filename);

    self.exports.push(
      JSON::new(config.data)
      .name(config.filename)
      .path(config.pathname)
      .parse()
    );

    if let Some(parent) = Path::new(&path).parent() {
      if !parent.exists() {
        fs::create_dir_all(parent)
        .unwrap_or_else(|_| panic!("Could not create {:?}", path));
      }
    }

    let mut file = fs::File::create(path)
    .expect("Could not create output file");

    file.write_all(self.exports[ExportMode::JSON as usize].as_bytes())
    .expect("Could not output to file");
  }
}
