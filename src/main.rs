use std::{cmp::min, ffi::OsStr, io, path::PathBuf};
use image::{self, DynamicImage, GenericImage, GenericImageView, Rgba, Rgb};
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
        print!("Image not square.");
        make_square(&mut image);
    }
    else { println!("Image is square");}
    
    let output_image: DynamicImage = image_to_power(&image);

    let output_path: PathBuf = get_save_dir(&input_path, extension);
    output_image.save(output_path).expect("Save failed");
}

fn get_open_dir() -> PathBuf
{
    FileDialog::new()
        .add_filter("image", &["png", "jpg","jpeg"])
        .pick_file()
        .expect("File selection failed")
}

fn get_save_dir(open_dir: &PathBuf, extension: &str) -> PathBuf
{
    let filename = open_dir.file_stem().unwrap().to_string_lossy();
    FileDialog::new()
        .set_directory(&open_dir)
        .set_file_name(&format!("{}Squared", filename))
        .add_filter("image", &vec![extension])
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




fn image_to_power(image: &DynamicImage) -> DynamicImage
{
    let mut output_img: DynamicImage = image::DynamicImage::new(image.width(), image.height(), image::ColorType::Rgb8);
    for img1_row in 0..image.height()
    {
        println!("1row {}", img1_row);
        let mut buffer: Vec<image::Rgb<u16>> = vec![];
        for img2_column in 0..image.width()
        {
            println!("  2col {}", img2_column);
            for img1_column in 0..image.width()
            {
                println!("      1col {}", img1_column);
                let colour1: Rgba<u8>  = image.get_pixel(img1_column, img1_row);
                
                let colour2: Rgba<u8> = image.get_pixel(img2_column, img1_column);

                let colour_out: image::Rgb<u16> = Rgb([
                    (colour1[0] as u16 * colour2[0] as u16), 
                    (colour1[1] as u16 * colour2[1] as u16), 
                    (colour1[2] as u16 * colour2[2] as u16)
                    ]);
                
                
                buffer.push(colour_out);
            }

            let pixel: Rgb<u32> = 
            buffer
                .iter()
                .fold(
                    Rgb([0,0,0]), 
                    |acc: Rgb<u32>, colour: &Rgb<u16>| 
                        Rgb([acc[0]+colour[0] as u32, 
                        acc[1]+colour[1] as u32, 
                        acc[2]+colour[2] as u32]));

            output_img
                .put_pixel(
                    img2_column,
                    img1_row, 
                    Rgba([pixel[0].clamp(0, 255) as u8, pixel[1].clamp(0, 255) as u8, pixel[2].clamp(0, 255) as u8, 255])
                );
            buffer.clear();
            println!("sent buffer")
        }
    }
    return output_img
}

fn input() -> String
{
    let mut text: String = String::new();
    io::stdin().read_line(&mut text).unwrap();
    return text
}