use pdfium_render::prelude::*;
use std::{fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), PdfiumError> {
    let mut pdfium_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pdfium_path.push("libpdfium.so");

    let pdfium = Pdfium::new(Pdfium::bind_to_library(pdfium_path)?);

    let document = pdfium.load_pdf_from_file("test/test3.pdf", None)?;

    let mut file = File::create("output.txt").expect("Failed to create output.txt");

    for (index, page) in document.pages().iter().enumerate() {
        writeln!(file, "=============== Page {} ===============", index).expect("Write failed");

        let text = page.text()?.all();
        writeln!(file, "{}", text).expect("Write failed");

        writeln!(file).unwrap();
    }

    Ok(())
}
