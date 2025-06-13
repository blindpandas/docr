use image::GenericImageView;
use std::error::Error;
use std::fmt;
use windows::{
    Globalization::{Language, LanguageLayoutDirection},
    Graphics::Imaging::{BitmapPixelFormat, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Security::Cryptography::CryptographicBuffer,
};

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(dead_code)]
    const IMAGE_DIMENTIONS: (u32, u32) = (1280, 720);

    #[test]
    fn test_it_extracts_text() -> DocrResult<()> {
        let text = recognize_image("en", "./assets/oz.jpg")?;
        assert!(text.trim().starts_with("You are welcome, most"));
        assert!(text.contains("Munchkins"));
        Ok(())
    }

    #[test]
    fn test_it_handles_rtl() -> DocrResult<()> {
        let text = recognize_image("ar", "./assets/rtl.jpg")?;
        let words: Vec<&str> = text.trim().split_ascii_whitespace().collect();
        assert_eq!(words[0], "هو");
        assert_eq!(words[1], "الكون");
        assert_eq!(words[words.len() - 2], "مهما");
        assert_eq!(words[words.len() - 1], "كبر");
        Ok(())
    }

    #[test]
    fn test_it_handles_invalid_language_tag() {
        let err_result = create_engine("aws");
        assert!(err_result.is_err());
    }
}

pub type DocrResult<T> = Result<T, DocrError>;
pub use DocrError::{OperationError, RuntimeError};

#[derive(Debug)]
pub enum DocrError {
    RuntimeError(String, i32),
    OperationError(String),
}

impl Error for DocrError {}

impl fmt::Display for DocrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self {
            RuntimeError(msg, code) => format!("Windows error: {} Code: {}.", msg, code),
            OperationError(msg) => format!("Error: {}", msg),
        };
        write!(f, "{}", err_message)
    }
}

impl From<windows::core::Error> for DocrError {
    fn from(error: windows::core::Error) -> Self {
        RuntimeError(error.message().to_string(), error.code().0)
    }
}

pub fn get_ocr_languages() -> DocrResult<Vec<String>> {
    let lang_tags: Vec<String> = OcrEngine::AvailableRecognizerLanguages()?
        .into_iter()
        .map(|lang| lang.LanguageTag().unwrap())
        .map(|tag| tag.to_string())
        .map(|tag| tag.to_ascii_lowercase())
        .collect();
    Ok(lang_tags)
}

pub fn recognize_imagedata(
    language: &str,
    imagedata: &[u8],
    width: i32,
    height: i32,
) -> DocrResult<String> {
    let bitmap = {
        let ibuf = CryptographicBuffer::CreateFromByteArray(imagedata)?;
        SoftwareBitmap::CreateCopyFromBuffer(ibuf, BitmapPixelFormat::Bgra8, width, height)?
    };
    Ok(recognize(language, bitmap)?)
}

pub fn recognize_image<'a>(language: &'a str, filename: &'a str) -> DocrResult<String> {
    let image = if let Ok(img) = image::open(filename) {
        img
    } else {
        let err_msg = format!("Failed to open image file: {}", filename);
        return Err(OperationError(err_msg));
    };
    let (width, height) = image.dimensions();
    let imagedata = image.to_rgba8().into_raw();
    let text = recognize_imagedata(language, &imagedata, width as i32, height as i32)?;
    Ok(text)
}

pub fn create_language_from_tag(given_tag: &str) -> DocrResult<Language> {
    let given_tag = given_tag.to_ascii_lowercase();
    let available_tags = get_ocr_languages()?;
    let valid_tag = if available_tags.contains(&given_tag) {
        Some(given_tag.clone())
    } else {
        available_tags
            .into_iter()
            .filter(|tag| tag.starts_with(&format!("{}-", given_tag)))
            .nth(0)
    };
    match valid_tag {
        Some(tag) => Ok(Language::CreateLanguage(tag)?),
        None => {
            let err_msg = format!(
                "Language '{}' is not supported by the OCR engine",
                &given_tag
            );
            return Err(OperationError(err_msg));
        }
    }
}

fn create_engine(lang_tag: &str) -> DocrResult<OcrEngine> {
    let lang = create_language_from_tag(lang_tag)?;
    Ok(OcrEngine::TryCreateFromLanguage(lang)?)
}

fn recognize(language: &str, bitmap: SoftwareBitmap) -> DocrResult<String> {
    let engine = create_engine(language)?;
    let lines: Vec<_> = engine
        .RecognizeAsync(bitmap)?
        .get()?
        .Lines()?
        .into_iter()
        .map(|line| line.Text().unwrap())
        .map(|hstr| hstr.to_string())
        .collect();
    let is_rtl = engine.RecognizerLanguage()?.LayoutDirection()? == LanguageLayoutDirection::Rtl;
    Ok(stringify_lines(lines, is_rtl))
}

fn stringify_lines(lines: Vec<String>, is_rtl: bool) -> String {
    lines.into_iter().fold(String::new(), |mut out, line| {
        if is_rtl {
            line.split_ascii_whitespace().rev().for_each(|word| {
                out.push_str(word);
                out.push(' ');
            });
        } else {
            out.push_str(line.trim_end());
        };
        out.push_str("\n");
        out
    })
}
