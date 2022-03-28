use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: {} <file>", args[0]);
    std::process::exit(2);
  }
  let file = &args[1];
  let image = rawler::decode_file(file).unwrap();

  // Write out the image as a PGM
  let mut f = BufWriter::new(File::create(format!("{}.pgm",file)).unwrap());
  let preamble = format!("P5 {} {} {}\n", image.width, image.height, 65535).into_bytes();
  f.write_all(&preamble).unwrap();
  if let rawler::RawImageData::Integer(data) = image.data {
    for pix in data {
      let pixhigh = (pix>>8) as u8;
      let pixlow  = (pix&0x0f) as u8;
      f.write_all(&[pixhigh, pixlow]).unwrap()
    }
  } else {
    eprintln!("Don't know how to process non-integer raw files");
  }
}
