mod items;
use items::ITEMS;
fn main() {
    for key in ITEMS.keys() {
        let item = &*ITEMS.get(key).unwrap();
        println!("{}", item);
    }
}
