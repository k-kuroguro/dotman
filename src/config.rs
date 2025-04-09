use serde::Deserialize;
use serde_yaml;
use std::{collections::HashMap, io::Read, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
   #[serde(deserialize_with = "deserialize_mappings")]
   pub mappings: Vec<Mapping>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Mapping {
   pub src: PathBuf,
   pub dest: PathBuf,
}

fn deserialize_mappings<'de, D>(deserializer: D) -> Result<Vec<Mapping>, D::Error>
where
   D: serde::Deserializer<'de>,
{
   let raw: HashMap<PathBuf, PathBuf> = HashMap::deserialize(deserializer)?;
   Ok(raw
      .into_iter()
      .map(|(src, dest)| Mapping { src, dest })
      .collect())
}

pub fn load_config_from_yaml<R: Read>(reader: R) -> Result<Config, serde_yaml::Error> {
   serde_yaml::from_reader(reader)
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_load_config_from_yaml() {
      let yaml = r#"
      mappings:
         bash/.bashrc: ~/.bashrc
         bash/.bash_aliases: ~/.bash_aliases
      "#;

      let config = load_config_from_yaml(yaml.as_bytes()).unwrap();
      assert_eq!(config.mappings.len(), 2);
      assert_eq!(
         config.mappings[0],
         Mapping {
            src: PathBuf::from("bash/.bashrc"),
            dest: PathBuf::from("~/.bashrc")
         }
      );
      assert_eq!(
         config.mappings[1],
         Mapping {
            src: PathBuf::from("bash/.bash_aliases"),
            dest: PathBuf::from("~/.bash_aliases")
         }
      );
   }
}
