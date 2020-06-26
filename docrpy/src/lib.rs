use docrapi as docr;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::prelude::*;

#[pyclass(module = "docrpy", weakref)]
#[text_signature = "(language: str)"]
struct DocrEngine {
    #[pyo3(get)]
    language: String,
}

#[pymethods]
impl DocrEngine {
    #[new]
    fn new(language: String) -> PyResult<Self> {
        match docr::create_language_from_tag(&language) {
            Ok(_) => Ok(DocrEngine { language }),
            Err(e) => {
                let err = match e {
                    docr::RuntimeError(..) => OSError::py_err(e.to_string()),
                    docr::OperationError(..) => ValueError::py_err(e.to_string()),
                };
                Err(err)
            }
        }
    }
    /// Return a list of language tags for languages supported by this OCR engine
    ///     DocrEngine.get_supported_languages() -> list
    #[staticmethod]
    fn get_supported_languages() -> PyResult<Vec<String>> {
        match docr::get_ocr_languages() {
            Ok(value) => Ok(value),
            Err(e) => {
                let err_msg = format!("Could not get  supported languages. {}", e);
                Err(RuntimeError::py_err(err_msg))
            }
        }
    }
    /// Run Optical Character Recognition (OCR) on the provided image data.
    ///     DocrEngine.recognize(self, imagedata: bytes, width: int, height: int) -> str
    #[text_signature = "(self, imagedata, width, height)"]
    fn recognize(&self, py: Python, imagedata: &[u8], width: i32, height: i32) -> PyResult<String> {
        py.allow_threads(|| {
            let result = docr::recognize_imagedata(&self.language, imagedata, width, height);
            if let Err(e) = result {
                let err_msg = format!("Error recognizing image data. {}", e);
                Err(OSError::py_err(err_msg))
            } else {
                Ok(result.unwrap())
            }
        })
    }
    /// Run OCR on the image file (supported formats: *.jpg, *.png).
    ///     DocrEngine.recognize_image_file(self, filename: str) -> str
    #[text_signature = "(self, filename, /)"]
    fn recognize_image_file(&self, py: Python, filename: &str) -> PyResult<String> {
        py.allow_threads(|| {
            let result = docr::recognize_image(&self.language, filename);
            if let Err(e) = result {
                let err_msg = format!("Error recognizing image '{}'. {}", &filename, e);
                Err(OSError::py_err(err_msg))
            } else {
                Ok(result.unwrap())
            }
        })
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for DocrEngine {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("DocrEngine(language='{}')", &self.language))
    }
}

#[pymodule]
fn docrpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DocrEngine>()?;
    Ok(())
}
