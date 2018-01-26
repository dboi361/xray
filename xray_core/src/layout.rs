use harfbuzz_sys;
use freetype::freetype;
use std::mem;
use std::ffi::CString;

struct FontLoader {
    raw_lib: freetype::FT_Library
}

impl FontLoader {
    fn new() -> Self {
        unsafe {
            let mut raw_lib = mem::uninitialized();
            freetype::FT_Init_FreeType(&mut raw_lib);
            Self { raw_lib }
        }
    }

    fn load_font_from_path(&self, path: &str) -> Font {
        unsafe {
            let path = CString::new(path).unwrap();
            let mut raw_font = mem::uninitialized();
            freetype::FT_New_Face(
                self.raw_lib,
                path.as_ptr(),
                0,
                &mut raw_font
            );

            Font { raw_font }
        }
    }
}

// TODO: impl drop for FontLoader

struct Font {
    raw_font: freetype::FT_Face
}

impl Font {
    fn set_size(&mut self, px: u16) {
        unsafe {
            freetype::FT_Set_Pixel_Sizes(
                self.raw_font,
                px as u32,
                px as u32
            );
        }
    }
}

// TODO: impl Drop for Font

struct LayoutBuffer {
    raw_buffer: *mut harfbuzz_sys::hb_buffer_t,
    length: u32
}

impl LayoutBuffer {
    pub fn new() -> Self {
        let raw_buffer = unsafe {
            harfbuzz_sys::hb_buffer_create()
        };
        Self { raw_buffer, length: 0 }
    }

    pub fn push_utf16_run(&mut self, run: &[u16]) {
        unsafe {
            harfbuzz_sys::hb_buffer_add_utf16(
                self.raw_buffer,
                run.as_ptr(),
                run.len() as i32,
                0,
                run.len() as i32
            );
        }
    }

    pub fn layout(self, font: &Font) {
        unsafe {
            let mut hb_font = harfbuzz_sys::hb_ft_font_create_referenced;
            harfbuzz_sys::hb_buffer_guess_segment_properties(self.raw_buffer);
        }
    }
}

impl Drop for LayoutBuffer {
    fn drop(&mut self) {
        unsafe {
            harfbuzz_sys::hb_buffer_destroy(self.raw_buffer)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_loading() {
        let loader = FontLoader::new();
        let mut font = loader.load_font_from_path("/Users/as-cii/Downloads/Inconsolata-Regular.ttf");
        font.set_size(24);


    }
}
