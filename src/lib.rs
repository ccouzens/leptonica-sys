#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::{pixFreeData, pixRead};
    use std::ffi::CString;

    #[test]
    fn image_size() {
        unsafe {
            let image = pixRead(CString::new("../test image.png").unwrap().as_ptr());
            assert_eq!((*image).w, 1000);
            assert_eq!((*image).h, 500);
            pixFreeData(image);
        }
    }
}
