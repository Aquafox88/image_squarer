use std::{cmp::min, ffi::OsStr, io::{self, stdin, Write}, path::PathBuf};
use image::{self, DynamicImage, GenericImage, GenericImageView, Rgba, Rgb};
use num::clamp;
use rfd::FileDialog;

fn main() 
{
    println!("Welcome to the image squarer.\nPress enter to start or '1' to open settings.");
    let selection = input();
    if selection.trim() == "1"
    {settings();}
    drop(selection);
    
    println!("Opening file selector dialog");
    let input_path: PathBuf = get_open_dir();
    let extension: &str = 
    input_path
        .extension()
        .and_then(OsStr::to_str)
        .expect("Filename failed to parse");

    let mut image = image::io::Reader::open(&input_path).expect("File failed to open")
        .with_guessed_format()
        .expect("File failed to open")
        .decode()
        .expect("File failed to open");
    println!("Image opened successfully");

    if image.width()!=image.height()
    {
        //print!("Image not square.");
        make_square(&mut image);
    }
    //else { println!("Image is square");}
    
    let output_image: DynamicImage = image_to_power(image);

    let output_path: PathBuf = get_save_dir(&input_path, extension);
    println!("Opening file save dialog");
    output_image.save(output_path).expect("Save failed");

    println!("Press enter to close");
    input();
}

fn settings() 
{
    let multithreading = "WIP";
    let hardware_acceleration = "WIP";

    println!("\nSettings:\n\nmultithreading: {}\nhardware acceleration: {}\nPress any button to exit", multithreading, hardware_acceleration);
    input();
}
fn get_open_dir() -> PathBuf
{
    FileDialog::new()
        .add_filter("image", &["png", "jpg","jpeg", "webp"])
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

fn get_pix_some_none(x: u16, y: u16, image: &DynamicImage) -> Option<Rgb<u8>>
{
    let mut pixel: Rgba<u8> = image.get_pixel(x as u32, y as u32);
    if pixel[3] == 0 {
        return None
    }
    let source: Vec<f32> = vec![
        pixel[0] as f32 / 255f32, 
        pixel[1] as f32 / 255f32, 
        pixel[2] as f32 /255f32, 
        pixel[3] as f32 / 255f32
    ];

    pixel[0] = clamp((((1f32 - source[3]) * 255f32) + (source[3] * source[0]) * 255f32) as u8, 0, 255);
    pixel[1] = clamp((((1f32 - source[3]) * 255f32) + (source[3] * source[1]) * 255f32) as u8, 0, 255);
    pixel[2] = clamp((((1f32 - source[3]) * 255f32) + (source[3] * source[2]) * 255f32) as u8,0, 255);

    let pixel_out: Rgb<u8> = Rgb([pixel[0], pixel[1], pixel[2]]);
    Some(pixel_out)
}

fn tonemap(pixel: Rgb<u32>, max_lum: u32) -> Rgba<u8>
{
    Rgba([(pixel[0] / max_lum * 255) as u8, (pixel[1] / max_lum * 255) as u8, (pixel[2] / max_lum * 255) as u8, 255])
}

fn image_to_power(image: DynamicImage) -> DynamicImage
{
    println!("");
    let mut output_img: DynamicImage = image::DynamicImage::new(image.width(), image.height(), image::ColorType::Rgb8);
    let max_lum = (255^2) * image.width();
    for img1_row in 0u16..image.height() as u16
    {
        //execute!(io::stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        print!("\rSquaring Image: row{} of {}", img1_row + 1, image.height());
        io::stdout().flush().unwrap();
        let mut buffer: Vec<image::Rgb<u16>> = vec![];
        for img2_column in 0u16..image.width() as u16
        {
            //println!("  2col {}", img2_column);
            for img1_column in 0u16..image.width() as u16
            {
                //println!("      1col {}", img1_column);
                let colour1: Rgb<u8>  = match get_pix_some_none(img1_column, img1_row, &image) {
                    Some(x) => x,
                    None => continue
                };

                let colour2: Rgb<u8>  = match get_pix_some_none(img2_column, img1_column, &image) {
                    Some(x) => x,
                    None => continue
                };

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
                    Rgb([
                        acc[0]+colour[0] as u32, 
                        acc[1]+colour[1] as u32, 
                        acc[2]+colour[2] as u32
                    ])
                );
            
            let pixel_out: Rgba<u8> = tonemap(pixel, max_lum);

            output_img
                .put_pixel(
                    img2_column as u32,
                    img1_row as u32, 
                    pixel_out
                );
            buffer.clear();
            //println!("sent buffer");
        }
    }
    print!("\nImage Squared successfully\n");

    output_img
}

fn input() -> String
{
    let mut text: String = String::new();
    stdin().read_line(&mut text).unwrap();
    return text
}