#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::response::content;

#[get("/")]
fn index(pics: State<PictureIndex>) -> content::Html<String> {
    //format!("Hello world! {} pics present", pics.pics.len())
    
    content::Html(format!("<html><body>{}</body></html>",
    pics.pics.iter().map(|p| String::from(p.to_str().unwrap())).collect::<Vec<String>>().join("<br>")))
}

use rocket::http::ContentType;
use std::path::PathBuf;

#[get("/image/<path..>")]
fn picture(path: PathBuf, pics: State<PictureIndex>) -> content::Content<&str> {
    println!("request for {:?}", path);
    content::Content(ContentType::HTML, "image goes here lawl")
}

struct PictureIndex {
    pics: Vec<std::path::PathBuf>
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![picture])
        .manage(PictureIndex { pics: init_pictures() })
        .launch();
}

use walkdir::WalkDir;

fn init_pictures() -> Vec<std::path::PathBuf> {
    let mut result: Vec<std::path::PathBuf> = Vec::new();
    let parent = std::path::Path::new("/home/jacob/Pictures");
    for entry in WalkDir::new(parent) {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
        match image::image_dimensions(path) {
            Ok(_) => result.push(path.strip_prefix(parent).unwrap().to_path_buf()),
            Err(e) => println!("Error parsing this one, {:?}", e)
        }
    }
    return result;
}

