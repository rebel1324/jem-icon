use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_text_mut};
use rusttype::{Font, Scale};
use std::path::Path;
use clap::{AppSettings, Clap};
use std::string::{String};
use std::char::{from_u32};

#[derive(Clap)]
#[clap(version="1.0", author="Kyu Yeon 'rebel' Lee <rebel1324@gmail.com>")]
#[clap(setting=AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short='o', long, default_value="output.png")]
    output: String,
    #[clap(short='u', long, default_value="e32d")]
    unicode: String,
    #[clap(short='x', long, default_value="8")]
    offsetx: i32,
    #[clap(short='y', long, default_value="8")]
    offsety: i32,
    #[clap(short='m', long, default_value="8")]
    margin: u32,
    #[clap(short='s', long, default_value="256")]
    size: u32,
    #[clap(short='S', long, default_value="0.65")]
    icon_scale: f32,
}

fn parse_unicode(s: &str) -> Option<char> {
    return u32::from_str_radix(s, 16).ok().and_then(from_u32);
}

fn main() {
    let opts = Opts::parse();

    // Validate fucking text.
    // todo: add pipe support or something.
    let mut text: String =String::new();
    let parse = parse_unicode(&opts.unicode);
    if let Some(x) = &parse {
        text.push(*x);
    } else {
        panic!("please can you provide more nicer unicode sequence? it should be something like xxxx");
    }

    let img_size: u32 = opts.size;
    let margin = opts.margin;
    let path = Path::new(&opts.output);
    let height = (img_size - margin * 2) as f32 * opts.icon_scale;
    let scale = Scale {
        x: height,
        y: height,
    };

    let mut image = RgbaImage::new(img_size, img_size);
    let font = Vec::from(include_bytes!("/usr/share/fonts/TTF/NotoSansMono-Light-Nerd-Font-Complete.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let circle_bg = Rgba([52, 152, 219, 255]);
    let white = Rgba([255u8, 255u8, 255u8, 255]);
    draw_filled_circle_mut(&mut image, 
        (
            (img_size as f32*0.5f32)as i32, 
            (img_size as f32*0.5f32) as i32
        ),
        (img_size as f32*0.45f32) as i32, 
        circle_bg);

    //image = imageproc::filter::gaussian_blur_f32(&image, 0.4f32);

    draw_text_mut(&mut image, white, 
        ((img_size as f32 * 0.5f32 - height / 2f32) as i32 + opts.offsetx) as u32,
        ((img_size as f32 * 0.5f32 - height / 2f32) as i32 + opts.offsety) as u32,
        scale, &font, &text);


    let _ = image.save(path).unwrap();
}
