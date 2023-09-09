use std::fs;
use std::io::Write;

/*
[15:42] raminfp:image_corpus (main *%) # file valid.png
valid.png: PNG image data, 1 x 1, 8-bit/color RGB, non-interlaced
[15:42] raminfp:image_corpus (main *%) # file invalid.png
invalid.png: data
 */
fn main() {

    let corpus_dir = "image_corpus";
    fs::create_dir_all(corpus_dir).unwrap();

    let mut file = fs::File::create(format!("{}/valid.png", corpus_dir)).unwrap();
    file.write_all(&valid_png_bytes()).unwrap();

    let mut file = fs::File::create(format!("{}/invalid.png", corpus_dir)).unwrap();
    file.write_all(&invalid_png_bytes()).unwrap();

    println!("Created image corpus in {}", corpus_dir);
}
// Returns valid minimal PNG file bytes
fn valid_png_bytes() -> Vec<u8> {

    // PNG signature
    let mut png = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    // IHDR chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52]);
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01]);
    png.extend_from_slice(&[0x08, 0x02, 0x00, 0x00, 0x00]);
    // IDAT chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54]);
    png.extend_from_slice(&[0x78, 0x01]);
    // IEND chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44]);
    png
}
/*
    RIFF and WAVE header
    fmt chunk with audio format info
    empty data chunk
 */
fn valid_audio_bytes() -> Vec<u8> {

    let mut wav = Vec::new();

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(44_i32.to_le_bytes())); // total size
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&(16_i32.to_le_bytes())); // chunk size
    wav.extend_from_slice(&(1_i16.to_le_bytes())); // PCM format
    wav.extend_from_slice(&(2_i16.to_le_bytes())); // stereo
    wav.extend_from_slice(&(44100_i32.to_le_bytes())); // sample rate
    wav.extend_from_slice(&(44100_i32.to_le_bytes())); // byte rate
    wav.extend_from_slice(&(4_i16.to_le_bytes())); // block align
    wav.extend_from_slice(&(16_i16.to_le_bytes())); // bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(44_i32.to_le_bytes())); // chunk size
    wav.extend_from_slice(&vec![0; 44]); // empty audio data

    wav
}
/*
    Header
    Object definitions
    xref table
    Trailer
    EOF
 */
fn valid_pdf_bytes() -> Vec<u8> {

    let mut pdf = vec![];

    // PDF header
    pdf.extend_from_slice(b"%PDF-1.7\n");

    // Object 1
    pdf.extend_from_slice(b"1 0 obj\n");
    pdf.extend_from_slice(b"<</Type/Catalog/Pages 2 0 R>>\n");
    pdf.extend_from_slice(b"endobj\n");

    // Object 2
    pdf.extend_from_slice(b"2 0 obj\n");
    pdf.extend_from_slice(b"<</Type/Pages/Kids[3 0 R]/Count 1>>\n");
    pdf.extend_from_slice(b"endobj\n");

    // Page object
    pdf.extend_from_slice(b"3 0 obj\n");
    pdf.extend_from_slice(b"<</Type/Page/Parent 2 0 R/MediaBox[0 0 3 3]>>\n");
    pdf.extend_from_slice(b"endobj\n");

    // xref table
    pdf.extend_from_slice(b"xref\n");
    pdf.extend_from_slice(b"0 4\n");
    pdf.extend_from_slice(b"0000000000 65535 f\n");
    pdf.extend_from_slice(b"0000000015 00000 n\n");
    pdf.extend_from_slice(b"0000000049 00000 n\n");
    pdf.extend_from_slice(b"0000000086 00000 n\n");

    // Trailer
    pdf.extend_from_slice(b"trailer\n");
    pdf.extend_from_slice(b"<</Size 4/Root 1 0 R>>\n");

    // EOF
    pdf.extend_from_slice(b"startxref\n");
    pdf.extend_from_slice(b"110\n");
    pdf.extend_from_slice(b"%%EOF\n");

    pdf
}
/*
    ftyp box
    moov container
    mvhd meta box
    mdat media data box
 */
fn valid_mp4_bytes() -> Vec<u8> {

    let mut mp4 = Vec::new();

    // ftyp box
    mp4.extend_from_slice(b"\0\0\0\024ftypmp42");
    mp4.extend_from_slice(b"\0\0\0\0mp42mp41");

    // moov box
    mp4.extend_from_slice(b"\0\0\0\024moov");

    // mvhd box
    mp4.extend_from_slice(b"\0\0\0\0\024mvhd\0\0\0\0");
    mp4.extend_from_slice(&(1_i32.to_be_bytes())); // version + flags

    // mdat box
    mp4.extend_from_slice(b"\0\0\0\016mdat\0\0\0\0");

    mp4
}
// Returns invalid PNG missing header
fn invalid_png_bytes() -> Vec<u8> {

    let mut png = vec![];
    // IHDR chunk
    png.extend_from_slice(&[0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52]);
    png
}