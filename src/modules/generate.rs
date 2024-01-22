use image::{ImageBuffer, Rgb, Rgb32FImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::error::Error;
use std::io::ErrorKind;

pub fn generate(
    bitmode: bool,
    gentype: bool,
    width: String,
    height: String,
    size: String,
    colors: Vec<Vec<f32>>,
    number: u16,
) -> Result<(), Box<dyn Error>> {
    // convert string to u32
    let width = width.parse::<u32>()?;
    let height = height.parse::<u32>()?;
    let size = size.parse::<u32>()?;

    if (width % size != 0) || (height % size != 0) {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::InvalidData,
            "image_width % box_size != 0",
        )));
    }

    // create imagebuf and color iterator
    let mut imagebuf32 = Rgb32FImage::new(width, height);
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
            draw_filled_rect_mut(&mut imagebuf32, rect, Rgb([color[0], color[1], color[2]]));
            let color = vec![
                (color[0] * 65_536f32) as u16,
                (color[1] * 65_536f32) as u16,
                (color[2] * 65_536f32) as u16,
            ];
            draw_filled_rect_mut(&mut imagebuf16, rect, Rgb([color[0], color[1], color[2]]));
        }
    }

    let path = std::env::current_exe()
        .ok()
        .and_then(|exe_dir| {
            exe_dir
                .to_str()
                .map(|path| format!("{}c3dlc/pics/", path.replace("converter3dlc", "")))
        })
        .unwrap_or_else(String::new);
    if bitmode {
        imagebuf32
            .save(format!("{path}output{number}_32bit.exr"))
            .unwrap();
    } else {
        imagebuf16
            .save(format!("{path}output{number}_16bit.png"))
            .unwrap();
    }
    Ok(())
}
