use escpos_rs::{EscposImage, Justification, Printer, PrinterProfile};
use photo::take_photo;
mod photo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We create the printer details
    let mut printer_details = PrinterProfile::usb_builder(0x4b43, 0x3830).build();
    // We pass it to the printer
    let printer = match Printer::new(printer_details) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :(")
        },
        Err(e) => panic!("Error: {}", e)
    };
    printer.println("Hello, world!")?;

    let jpg = take_photo()?;
    let printer_image = EscposImage::new(jpg, 128, Justification::Center).unwrap();

    printer.cut()?;

    Ok(())
}
