use image::DynamicImage;
use v4l::{buffer::Type, context, io::mmap::Stream, Device};
use v4l::io::traits::CaptureStream;
use image::{codecs::jpeg::JpegDecoder};
use v4l::video::Capture;


pub fn take_photo() -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let devices = context::enum_devices();
    let webcam = devices
        .iter()
        .rev()
        .find(|dev| dev.name().unwrap().contains("Webcam"))
        .unwrap();

    println!("Device: {} ({})", webcam.name().unwrap(), webcam.index());
    
    let mut device = Device::new(webcam.index()).unwrap();
    println!("Device opened");
    
    let mut stream = Stream::with_buffers(&mut device, Type::VideoCapture, 4)
        .expect("Failed to create buffer stream");
    println!("Staring stream");
    let (buf, meta) = stream.next().unwrap();
    println!(
        "Buffer size: {}, seq: {}, timestamp: {}",
        buf.len(),
        meta.sequence,
        meta.timestamp
    );
    return Ok(decode_jpg(buf)?);
}

pub fn decode_jpg(jpg_img: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let decoder = JpegDecoder::new(jpg_img)?;
    let img = DynamicImage::from_decoder(decoder)?;

    Ok(img)
}
