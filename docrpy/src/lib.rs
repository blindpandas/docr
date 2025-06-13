use docrapi as docr;
use pyo3::exceptions::*;
use pyo3::prelude::*;

#[pyclass(frozen, module = "docrpy", weakref)]
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
                    docr::RuntimeError(..) => PyOSError::new_err(e.to_string()),
                    docr::OperationError(..) => PyValueError::new_err(e.to_string()),
                };
                Err(err)
            }
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("DocrEngine(language='{}')", &self.language))
    }

    /// Return a list of language tags for languages supported by this OCR engine
    ///     DocrEngine.get_supported_languages() -> list
    #[staticmethod]
    fn get_supported_languages() -> PyResult<Vec<String>> {
        match docr::get_ocr_languages() {
            Ok(value) => Ok(value),
            Err(e) => {
                let err_msg = format!("Could not get  supported languages. {}", e);
                Err(PyRuntimeError::new_err(err_msg))
            }
        }
    }

    /// Run Optical Character Recognition (OCR) on the provided image data.
    ///     DocrEngine.recognize(self, imagedata: bytes, width: int, height: int) -> str
    #[pyo3(text_signature = "(self, imagedata, width, height)")]
    fn recognize(&self, py: Python, imagedata: &[u8], width: i32, height: i32) -> PyResult<String> {
        py.allow_threads(|| {
            let result = docr::recognize_imagedata(&self.language, imagedata, width, height);
            if let Err(e) = result {
                let err_msg = format!("Error recognizing image data. {}", e);
                Err(PyOSError::new_err(err_msg))
            } else {
                Ok(result.unwrap())
            }
        })
    }

    /// Run OCR on the image file (supported formats: *.jpg, *.png).
    ///     DocrEngine.recognize_image_file(self, filename: str) -> str
    #[pyo3(text_signature = "(self, filename, /)")]
    fn recognize_image_file(&self, py: Python, filename: &str) -> PyResult<String> {
        py.allow_threads(|| {
            let result = docr::recognize_image(&self.language, filename);
            if let Err(e) = result {
                let err_msg = format!("Error recognizing image '{}'. {}", &filename, e);
                Err(PyOSError::new_err(err_msg))
            } else {
                Ok(result.unwrap())
            }
        })
    }
}

#[pymodule]
fn docrpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DocrEngine>()?;
    Ok(())
}
