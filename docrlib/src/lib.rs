use docrapi::*;
use libc::wchar_t;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uchar, c_ulong};
use std::slice;
use widestring::U16CString;

#[unsafe(no_mangle)]
pub extern "C" fn get_recognizable_languages(out_ptr: *mut wchar_t) -> i32 {
    match get_ocr_languages() {
        Ok(langs) => {
            let data = serde_json::to_string(&langs).unwrap_or("[]".to_string());
            unsafe { write_unicode_to_buffer(&data, out_ptr, "[]") };
            0
        }
        Err(e) => get_error_code(e),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn recognize_image(
    lang_ptr: *const c_char,
    buf_ptr: *const c_uchar,
    width: c_ulong,
    height: c_ulong,
    out_ptr: *mut wchar_t,
) -> i32 {
    if lang_ptr.is_null() || buf_ptr.is_null() {
        return -1;
    };
    let lang = unsafe { CStr::from_ptr(lang_ptr).to_str().unwrap_or("en") };
    let (width, height) = (width as i32, height as i32);
    let buf_len = width * height * 4;
    let imagedata = unsafe { slice::from_raw_parts(buf_ptr, buf_len as usize) };
    match recognize_imagedata(lang, imagedata, width, height) {
        Ok(text) => {
            if out_ptr.is_null() {
                return -1;
            };
            unsafe { write_unicode_to_buffer(&text, out_ptr, "") };
            0
        }
        Err(e) => get_error_code(e),
    }
}

unsafe fn write_unicode_to_buffer(source_str: &str, out_ptr: *mut wchar_t, default_value: &str) {
    unsafe {
        let buf = U16CString::from_str(source_str)
            .unwrap_or_else(|_| U16CString::from_str_unchecked(default_value));
        out_ptr.copy_from_nonoverlapping(buf.as_ptr(), buf.len());
    }
}

fn get_error_code(err: DocrError) -> i32 {
    match err {
        RuntimeError(_, code) => code as i32,
        OperationError(_) => -1,
    }
}
