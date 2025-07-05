use core::ffi::c_char;

#[repr(C, align(8))]
pub struct ElfInterp([c_char;24]);

#[unsafe(link_section = ".interp")]
#[unsafe(export_name = "fake_interp")]
pub static FAKE_INTERP: ElfInterp = {
    // Explicitly filling the rest of array past text with zeroes.
    let text = *b"/nonexistent/masami/";
    let mut arr: [c_char;24] = [0;24];
    let mut i = 0;
    while i < text.len() {
        arr[i] = text[i] as i8;
        i+=1;
    }
    ElfInterp(arr)
};
