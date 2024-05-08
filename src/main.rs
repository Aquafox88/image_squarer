use std::{cmp::min, ffi::OsStr, fmt::format, io, path::PathBuf};
use image::{self, DynamicImage, GenericImage, GenericImageView, Pixel};
use rfd::FileDialog;

fn main() 
{
    let input_path: PathBuf = get_open_dir();
    let extension: &str = 
    input_path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Filename failed to parse");

    let mut image: DynamicImage = image::open(&input_path).expect("File failed to open");

    if image.width()!=image.height()
    {
        make_square(&mut image);
    }
    
    let output_path: PathBuf = get_save_dir(&input_path, extension);

    image.save(output_path).expect("Save failed");
}

fn get_open_dir() -> PathBuf
{
    FileDialog::new()
        .add_filter("text", &["png", "jpg","jpeg"])
        .pick_file()
        .expect("File selection failed")
}

fn get_save_dir(open_dir: &PathBuf, extension: &str) -> PathBuf
{
    FileDialog::new()
        .set_directory(&open_dir)
        .set_file_name(&open_dir.file_name().unwrap())
        .save_file()
        .expect("File saving failed")
}

fn make_square(image: &mut DynamicImage)
{
    let width: u32 = min(image.width(), image.height());

    let (x, y) = 
    if image.width() > image.height() 
    { ((image.width() / 2) - (image.height() / 2) as u32, 0) } 
    else 
    { (0, (image.height() / 2) - (image.width() / 2) as u32) };
    
    *image = image.crop_imm(x, y, width, width);
}




fn square(image: &DynamicImage) -> DynamicImage
{
    let mut output_img: DynamicImage = image::DynamicImage::new(image.width(), image.height(), image::ColorType::Rgb8);
    let colour: image::Rgba<u8> = image.get_pixel(0, 0);
    output_img.put_pixel(0, 0, colour);
    println!("{:?}", colour);
    return output_img
}

fn input() -> String
{
    let mut text: String = String::new();
    io::stdin().read_line(&mut text).unwrap();
    return text
}