use colored::Colorize;
use ron::error::SpannedResult;
use std::{collections::HashMap, fs, fs::File, io::Read, result::Result};

pub trait FactorioType {
    fn get_id_hash(&self) -> u64;
    fn get_id(&self) -> &String;
    fn get_name(&self) -> &String;
    fn get_proto_hash(&self) -> u64;
    fn get_proto(&self) -> &String;
    fn get_icon(&self) -> &String;
}

pub trait DynamicDeserialize {
    fn deserialize(map: HashMap<String, String>) -> Result<Self, String>
    where
        Self: Sized;
}

pub fn read_to_hashmap<'a, T: FactorioType + DynamicDeserialize>(
    directory: &str,
) -> HashMap<u64, T> {
    let mut map = HashMap::new();
    let paths = fs::read_dir(directory);
    if let Ok(paths) = paths {
        for path in paths {
            if let Ok(path) = path {
                let result = path.path();
                let result = result.to_str();
                let path = path.path();
                let path_str: &str;
                if let Some(p) = result {
                    path_str = p;
                } else {
                    println!(
                        "{}",
                        "Unable to get displayable file path for debug output".truecolor(255, 195, 63)
                    );
                    path_str = "<unknown directory>";
                }
                let file = File::open(path);
                if let Ok(mut file) = file {
                    let mut input = String::new();
                    let result = file.read_to_string(&mut input);
                    if let Ok(_) = result {
                        let list: SpannedResult<Vec<HashMap<String, String>>> =
                            ron::from_str(&input);
                        if let Ok(list) = list {
                            for item in list {
                                let item = T::deserialize(item);
                                match item {
                                    Ok(item) => {
                                        if let Some(old) = map.insert(item.get_id_hash(), item) {
                                            println!(
                                                "{}{}{}{}",
                                                "Duplicate entry found for ".truecolor(255, 195, 63),
                                                old.get_id().bright_yellow(),
                                                " when reading ".truecolor(255, 195, 63),
                                                path_str.bright_yellow(),
                                            );
                                        }
                                    }
                                    Err(error) => println!(
                                        "{}{}{}{}",
                                        "Encountered error while parsing ".truecolor(255, 195, 63),
                                        path_str.bright_yellow(),
                                        ": ".truecolor(255, 195, 63),
                                        error
                                    ),
                                }
                            }
                        } else {
                            println!(
                                "{}{}{}",
                                "Unable to parse ".truecolor(255, 195, 63),
                                path_str.bright_yellow(),
                                ". Possible syntax or spelling error".truecolor(255, 195, 63),
                            );
                        }
                    } else {
                        println!(
                            "{}{}{}",
                            "Unable to read contents of ".truecolor(255, 195, 63),
                            path_str.bright_yellow(),
                            " as a string".truecolor(255, 195, 63)
                        );
                    }
                } else {
                    println!("Unable to open {}", path_str);
                }
            }
        }
    }
    map
}
