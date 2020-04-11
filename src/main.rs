#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}



fn main() {
    init_pictures();
    rocket::ignite().mount("/", routes![index]).launch();
}

use walkdir::WalkDir;

fn init_pictures() -> Vec<std::path::PathBuf> {
    let mut result: Vec<std::path::PathBuf> = Vec::new();
    for entry in WalkDir::new("/home/jacob/Pictures") {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
        match image::image_dimensions(path) {
            Ok(_) => result.push(path.to_path_buf()),
            Err(e) => println!("Error parsing this one, {:?}", e)
        }
    }
    return result;
}

