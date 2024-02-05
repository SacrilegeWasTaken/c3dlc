use image::{ImageBuffer, Rgb, Rgb32FImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::error::Error;
use std::io::ErrorKind;

pub fn generate(
    bitmode:    bool,
    gentype:    bool,
    width:      String,
    height:     String,
    size:       String,
    colors:     Vec<Vec<f32>>,
    number:     u16,
) -> Result<(), Box<dyn Error>>
{
    let width = width.parse::<u32>()?;
    let height = height.parse::<u32>()?;
    let size = size.parse::<u32>()?;

    if size == 0 {
        return Err(Box::new(std::io::Error::new(ErrorKind::WriteZero, "")));
    }
    if (width % size != 0) || (height % size != 0) {
        return Err(Box::new(std::io::Error::new(ErrorKind::InvalidData, "")));
    }

    match bitmode {
        true => {
            std::thread::spawn(move || generate32bit(gentype, width, height, size, colors, number))
        }
        false => {
            std::thread::spawn(move || generate16bit(gentype, width, height, size, colors, number))
        }
    };

    Ok(())
}

fn generate32bit(
    gentype:    bool,
    width:      u32,
    height:     u32,
    size:       u32,
    colors:     Vec<Vec<f32>>,
    number:     u16,
)
{
    let mut imagebuf32 = Rgb32FImage::new(width, height);
    let mut color = colors.iter();
    for i in 0..(height / size) {
        for j in 0..(width / size) {
            let color = match gentype {
                true => match color.next() {
                    Some(c) => c,
                    None => {
                        color = colors.iter();
                        color.next().unwrap()
                    }
                },
                false => match color.next() {
                    Some(c) => c,
                    None => break,
                },
            };
            let rect = Rect::at((j * size) as i32, (i * size) as i32).of_size(size, size);
            draw_filled_rect_mut(&mut imagebuf32, rect, Rgb([color[0], color[1], color[2]]));
        }
    }

    image::imageops::flip_vertical_in_place(&mut imagebuf32);
    imagebuf32.save(get_save_path(number, true)).unwrap();
}


fn generate16bit(
    gentype:    bool,
    width:      u32,
    height:     u32,
    size:       u32,
    colors:     Vec<Vec<f32>>,
    number:     u16,
)
{
    let mut imagebuf16 = ImageBuffer::new(width, height);
    let mut color = colors.iter();
    for i in 0..(height / size) {
        for j in 0..(width / size) {
            let color = match gentype {
                true => match color.next() {
                    Some(c) => c,
                    None => {
                        color = colors.iter();
                        color.next().unwrap()
                    }
                },
                false => match color.next() {
                    Some(c) => c,
                    None => break,
                },
            };
            let rect = Rect::at((j * size) as i32, (i * size) as i32).of_size(size, size);
            let color = vec![
                (color[0] * 65_536f32) as u16,
                (color[1] * 65_536f32) as u16,
                (color[2] * 65_536f32) as u16,
            ];
            draw_filled_rect_mut(&mut imagebuf16, rect, Rgb([color[0], color[1], color[2]]));
        }
    }
    image::imageops::flip_vertical_in_place(&mut imagebuf16);
    imagebuf16.save(get_save_path(number, false)).unwrap();
}


fn get_save_path(number: u16, bitmode: bool) -> String
{
    let path;

    if cfg!(target_os = "windows"){
        path = std::env::current_exe()
            .ok()
            .and_then(|exe_dir| {
                exe_dir
                    .to_str()
                    .map(|path| format!("{}c3dlc/pics/", path.replace("converter3dlc", "")))
            })
            .unwrap_or_else(String::new);
    } else {
        path = std::env::current_exe()
            .ok()
            .and_then(|exe_dir| {
                exe_dir
                    .to_str()
                    .map(|path| format!("{}c3dlc/pics/", path.replace("converter3dlc", "")))
            })
            .unwrap_or_else(String::new);
    }
    match bitmode {
        true => return format!("{path}output{number}_32bit.exr"),
        false => return format!("{path}output{number}_16bit.png"),
    };
}
