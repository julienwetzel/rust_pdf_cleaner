use image::{DynamicImage, ImageOutputFormat};
use mupdf::{Document, Error};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn convert_pdf_to_tiff<P: AsRef<Path>>(input_file: P, output_dir: P) -> Result<(), Error> {
    // Charger le document PDF
    let doc = Document::open(input_file)?;

    // Parcourir les pages du document
    for i in 0..doc.page_count() {
        // Récupérer la page du document
        let page = doc.load_page(i)?;

        // Rendre la page en image avec une résolution de 600 DPI
        let img = page.render_pixmap(600, 600, None)?;

        // Créer une DynamicImage pour l'image TIFF
        let img = DynamicImage::ImageRgba8(img.to_image()?);

        // Créer le fichier de sortie pour l'image TIFF
        let output_file = output_dir.as_ref().join(format!("page_{}.tiff", i));
        let file = File::create(output_file)?;
        let ref mut w = BufWriter::new(file);

        // Enregistrer l'image au format TIFF
        img.write_to(w, ImageOutputFormat::Tiff)?;
    }

    Ok(())
}

fn main() {
    let input_file = "input.pdf";
    let output_dir = "./";

    if let Err(e) = convert_pdf_to_tiff(input_file, output_dir) {
        eprintln!("Erreur lors de la conversion du PDF en TIFF : {:?}", e);
    } else {
        println!("Conversion réussie !");
    }
}
