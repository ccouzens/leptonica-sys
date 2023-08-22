#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Do not warn on u128 types in the C FFI.
// This can be observed on x86-64 Linux, such as on the GitHub Actions.
// https://github.com/ccouzens/leptonica-sys/issues/3
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[allow(improper_ctypes)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(any(not(target_os = "linux"), not(target_arch = "x86_64")))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::{pixFreeData, pixGetHeight, pixGetWidth, pixRead};

    #[test]
    fn image_size() {
        unsafe {
            let image = pixRead(b"../test image.png\0".as_ptr().cast());
            assert_eq!(pixGetWidth(image), 1000);
            assert_eq!(pixGetHeight(image), 500);
            pixFreeData(image);
        }
    }

    #[test]
    #[allow(path_statements)]
    fn defined_constants() {
        super::IFF_UNKNOWN;
        super::IFF_BMP;
    }
}
