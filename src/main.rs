use clap::Parser;
use std::path::Path;
use webp::*;

use log::info;

#[derive(Parser, Debug)]
#[clap(name = "Rust webp image converter")]
#[clap(author = "Jeff Mitchell <sentinel1909@protonmail.com>")]
#[clap(version = "0.1.0")]
#[clap(about = "Converts an image in any format to webp")]
struct Args {
    #[clap(short, long, value_parser)]
    inputfile: String,
    #[clap(short, long, value_parser)]
    outputfile: String,
    #[clap(short, long, value_parser)]
    quality: f32,
}

fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = Args::parse();
    let inputpath = Path::new(&args.inputfile);
    let outputpath = Path::new(&args.outputfile);
    let input_img_result = image::open(&inputpath);
    let input_img = match input_img_result {
        Ok(img) => img,
        Err(error) => panic!("problem opening the image file: {:?}", error),
    };
    let encoder: Encoder = Encoder::from_image(&input_img).unwrap();
    let webp: WebPMemory = encoder.encode(args.quality);
    std::fs::write(&outputpath, &*webp)?;
    info!("{:?}", "Conversion to webp format successful.");
    Ok(())
}
