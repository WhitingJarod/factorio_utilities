use lazy_static::lazy_static;
use ron::error::SpannedResult;
use std::{collections::HashMap, fs, fs::File, io::Read};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub mod item;
use item::{Item, SerializableItem};

lazy_static! {
    pub static ref ITEMS: HashMap<u64, Item> = {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let result = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 195, 63))));
        if let Err(_) = result {
            println!("Unable to set output color with termcolor.");
        }
        let mut map = HashMap::new();
        let paths = fs::read_dir("./resources/item_definitions/");
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
                        println!("Unable to get displayable file path for debug output.");
                        path_str = "<unknown directory>";
                    }
                    let file = File::open(path);
                    if let Ok(mut file) = file {
                        let mut input = String::new();
                        let result = file.read_to_string(&mut input);
                        if let Ok(_) = result {
                            let list: SpannedResult<Vec<SerializableItem>> = ron::from_str(&input);
                            if let Ok(list) = list {
                                for item in list {
                                    let item = item.to_item();
                                    if let Some(old) = map.insert(item.uuid, item) {
                                        println!(
                                            "Duplicate entry found for {} when reading {}",
                                            old.id, path_str
                                        );
                                    }
                                }
                            } else {
                                println!(
                                    "Unable to parse {}. Possible syntax or spelling error.",
                                    path_str
                                );
                            }
                        } else {
                            println!("Unable to read contents of {} as a string.", path_str);
                        }
                    } else {
                        println!("Unable to open {}", path_str);
                    }
                }
            }
            let result = stdout.set_color(ColorSpec::new().set_fg(None));
            if let Err(_) = result {
                println!("Unable to set output color with termcolor.");
            }
        }
        map
    };
    pub static ref ITEM_TYPES: Vec<u64> = {
        let mut list = Vec::new();
        for key in ITEMS.keys() {
            list.push(*key);
        }
        list
    };
}
