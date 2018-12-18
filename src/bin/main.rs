// Contains the incoming JSON schema -> Types mapping
extern crate danger;

fn main() {
    let danger_dsl = danger::get_danger();
    let created = danger_dsl.git.created_files;

    println!("New Files! {:#?}", created);
}
