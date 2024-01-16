use bson;
use std::io::Write;
use brotli;

fn brotli_compress(input: &[u8]) -> Vec<u8> {
    let mut writer = brotli::CompressorWriter::new(
        Vec::new(),
        4096,
        11,
        22);
    writer.write_all(input).unwrap();
    writer.into_inner()
}

fn brotli_decompress(input: &[u8]) -> Vec<u8> {
    let mut writer = brotli::DecompressorWriter::new(
        Vec::new(),
        4096);
    writer.write_all(input).unwrap();
    writer.into_inner().unwrap()
}


pub fn compress(json: String) -> Vec<u8> {
    let bson_data = bson::to_bson(&json).unwrap().to_string();
    let mut compressed_data = brotli_compress(bson_data.as_bytes());
    let mut header:Vec<u8> = vec![0x46, 0x72, 0x44, 0x54, 0x00, 0x00, 0x00, 0x00, 0x03];
    header.append(&mut compressed_data);
    return header;
}

pub fn decompress(brson: Vec<u8>) -> String {
    let data = &brson[9..];
    let decompressed_data =  brotli_decompress(data);
    let bson_data:Vec<u8> = bson::from_slice(&decompressed_data).unwrap();
    return String::from_utf8(bson_data).unwrap();

}
