#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::State;

#[get("/")]
fn index(pics: State<PictureIndex>) -> String {
    format!("Hello world! {} pics present", pics.pics.len())
}

struct PictureIndex {
    pics: Vec<std::path::PathBuf>
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .manage(PictureIndex { pics: init_pictures() })
        .launch();
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

