// https://github.com/Marwes/schemafy/blob/master/src/lib.rs

extern crate reqwest;
extern crate schemafy;

use std::io::Read;

// Note that I have done a LOT of hand-edits to the
// file to make it compile, so expect a bit of work making it work
//
// Currently:
//  - Remove all optional types
//  - Remove some uncompiling bitbucket issues
//
fn main() -> Result<(), Box<std::error::Error>> {
  let url = "https://raw.githubusercontent.com/danger/danger-js/master/source/danger-incoming-process-schema.json";
  let mut resp = reqwest::get(url)?;

  let mut schema = String::new();
  resp.read_to_string(&mut schema);

  let output = schemafy::generate(Some("Schema"), &schema)?;
  std::fs::write("src/dsl.rs", output.as_bytes())?;
  Ok(())
}
