use std::sync::Once;
use tiff::encoder::compression::DeflateLevel::Balanced;

use crate::cleanup::remove_compressed_test_file;
mod cleanup;

static INIT: Once = Once::new();

pub fn initialize(file: &str) {
    INIT.call_once(|| {
        remove_compressed_test_file(file);
    });
}

#[test]
fn rgb8_uncompressed() {
    let output = "tests/samples/output/uncompressed_rgb8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Uncompressed;
    caesium::compress(
        String::from("tests/samples/rgb8.tif"),
        String::from(output),
        &params,
    )
    .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgba8_uncompressed() {
    let output = "tests/samples/output/uncompressed_rgba8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Uncompressed;
    caesium::compress(
        String::from("tests/samples/rgba8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgb8_deflate() {
    let output = "tests/samples/output/deflate_rgb8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Deflate;
    params.tiff.deflate_level = Balanced;
    caesium::compress(
        String::from("tests/samples/rgb8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgba8_deflate() {
    let output = "tests/samples/output/deflate_rgba8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Deflate;
    params.tiff.deflate_level = Balanced;
    caesium::compress(
        String::from("tests/samples/rgba8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgb8_lzw() {
    let output = "tests/samples/output/lzw_rgb8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Lzw;
    caesium::compress(
        String::from("tests/samples/rgb8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgba8_lzw() {
    let output = "tests/samples/output/lzw_rgba8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Lzw;
    caesium::compress(
        String::from("tests/samples/rgba8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgb8_packbits() {
    let output = "tests/samples/output/packbits_rgb8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Packbits;
    caesium::compress(
        String::from("tests/samples/rgb8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgba8_packbits() {
    let output = "tests/samples/output/packbits_rgba8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Packbits;
    caesium::compress(
        String::from("tests/samples/rgba8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    remove_compressed_test_file(output)
}

#[test]
fn rgb8_downscale() {
    let output = "tests/samples/output/downscale_rgb8.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Lzw;
    params.width = 50;
    params.height = 20;
    caesium::compress(
        String::from("tests/samples/rgb8.tif"),
        String::from(output),
        &params,
    )
        .unwrap();
    assert!(std::path::Path::new(output).exists());
    assert_eq!(
        infer::get_from_path(output).unwrap().unwrap().mime_type(),
        "image/tiff"
    );
    assert_eq!(image::image_dimensions(output).unwrap(), (50, 20));
    remove_compressed_test_file(output)
}

#[test]
fn unsupported() {
    let output = "tests/samples/output/unsupported.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Lzw;
    assert!(caesium::compress(
        String::from("tests/samples/unsupported.tif"),
        String::from(output),
        &params,
    ).is_err());
}

#[test]
fn prevent_panic() {
    let output = "tests/samples/output/panic.tif";
    initialize(output);
    let mut params = caesium::initialize_parameters();
    params.tiff.algorithm = caesium::TiffCompression::Lzw;
    assert!(caesium::compress(
        String::from("tests/samples/unsupported.tif"),
        String::from(output),
        &params,
    ).is_err());
}