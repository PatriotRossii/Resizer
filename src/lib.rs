use std::ops::Deref;
use std::path::PathBuf;

use image::io::Reader;

#[derive(Copy, Clone, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl From<&str> for Resolution {
    fn from(source: &str) -> Self {
        let (width, height) = {
            let dimensionals: Vec<&str> = source.split("x").collect();

            if let Some(width) = dimensionals.get(0) {
                if let Some(height) = dimensionals.get(1) {
                    (
                        width
                            .parse::<u32>()
                            .expect("Failed to parse resolution's width"),
                        height
                            .parse::<u32>()
                            .expect("Failed to parse resolution's height"),
                    )
                } else {
                    panic!("Invalid resolution height")
                }
            } else {
                panic!("Invalid resolution width")
            }
        };
        Resolution { width, height }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FilterType(image::imageops::FilterType);

impl Deref for FilterType {
    type Target = image::imageops::FilterType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for FilterType {
    fn from(source: &str) -> Self {
        match source {
            "nearest" => FilterType(image::imageops::FilterType::Nearest),
            "triangle" => FilterType(image::imageops::FilterType::Triangle),
            "catmullrom" => FilterType(image::imageops::FilterType::CatmullRom),
            "gaussian" => FilterType(image::imageops::FilterType::Gaussian),
            "lanczos3" => FilterType(image::imageops::FilterType::Lanczos3),
            _ => unreachable!(),
        }
    }
}

fn generate_output_path(original_path: &PathBuf, output_path: &PathBuf) -> PathBuf {
    let mut result = output_path.clone();

    result.push(&format!(
        "{}-resized.{}",
        original_path
            .file_stem()
            .expect(&format!(
                "Failed to get name of the file: {}",
                original_path.to_str().unwrap()
            ))
            .to_str()
            .unwrap(),
        original_path
            .extension()
            .expect(&format!(
                "Failed to get extension of the file: {}",
                original_path.to_str().unwrap()
            ))
            .to_str()
            .unwrap()
    ));
    result
}

pub fn user_error(err: &str) {
    println!("error: {}", err);
    std::process::exit(1);
}

pub fn open_resize_save<'a, T>(
    iterator: T,
    filter: FilterType,
    resolution: Resolution,
    output_path: &PathBuf,
) -> Result<(), String>
where
    T: std::iter::Iterator<Item = &'a PathBuf>,
{
    for path in iterator {
        let path_str = path.to_str().unwrap();

        let image = Reader::open(path)
            .expect(&format!("Failed to open an image: {}", path_str))
            .with_guessed_format()
            .expect(&format!(
                "Failed to detect format of the image: {}",
                path_str
            ))
            .decode()
            .expect(&format!("Failed to decode the image: {}", path_str));

        let new_image = image.resize_exact(resolution.width, resolution.height, *filter);
        let save_path = generate_output_path(&path, &output_path);
        if let Err(e) = new_image.save(&save_path) {
            return Err(format!(
                "Failed to save an image at the path {:?}: {}",
                save_path.into_os_string(),
                e
            ));
        }
    }
    Ok(())
}
