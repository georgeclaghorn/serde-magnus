use magnus::{eval, Error, RHash};
use serde_magnus::deserialize;
use std::collections::HashMap;

#[test]
fn test_deserializing_maps() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let input: RHash = eval!(
        r#"
        {
          "Yes" => "No",
          "Stop" => "Go",
          "High" => "Low"
        }
    "#
    )?;

    let output: HashMap<String, String> = deserialize(input)?;
    assert_eq!(3, output.len());
    assert_eq!(Some(&"No".into()), output.get("Yes"));
    assert_eq!(Some(&"Go".into()), output.get("Stop"));
    assert_eq!(Some(&"Low".into()), output.get("High"));

    Ok(())
}
