use resizer::FilterType;
use resizer::Resolution;

use clap::{App, Arg};
use std::fs::read_dir;
use std::path::PathBuf;

fn main() {
    let matches = App::new("Image Resizer")
        .version("0.1")
        .author("Eoan Ermine <patriotrossii2019@mail.ru>")
        .about("Resizes images")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("PATH")
            .help("Sets a path to image/directory contains images we want to resize")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("resolution")
            .short("r")
            .long("resolution")
            .value_name("RESOLUTION")
            .help("Sets a resolution to what we want to convert images")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("filter")
            .short("f")
            .long("filter")
            .value_name("FILTER")
            .help("Defines filter type. Available filters: nearest, triangle, catmullrom, gaussian, lanczos3")
            .default_value("nearest")
            .possible_values(&["nearest", "triangle", "catmullrom", "gaussian", "lanczos3"]))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .help("Output directory")
            .takes_value(true)
            .required(true))
        .get_matches();

    let path = PathBuf::from(matches.value_of("path").unwrap());
    let resolution = Resolution::from(matches.value_of("resolution").unwrap());
    let output_path = PathBuf::from(matches.value_of("output").unwrap());

    let filter = FilterType::from(matches.value_of("filter").unwrap());

    if path.is_dir() == true {
        let paths: Vec<PathBuf> = read_dir(&path)
            .expect("Failed to open folder with images")
            .map(|entry| entry.unwrap().path())
            .collect();
        if let Err(e) = resizer::open_resize_save(paths.iter(), filter, resolution, &output_path) {
            resizer::user_error(&e);
        }
    }

    if path.is_file() == true {
        if let Err(e) =
            resizer::open_resize_save(std::iter::once(&path), filter, resolution, &output_path)
        {
            resizer::user_error(&e);
        }
    }
}
