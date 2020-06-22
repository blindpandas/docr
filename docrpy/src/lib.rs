use docrapi as docr;
use pyo3::class::PyObjectProtocol;
use pyo3::exceptions::*;
use pyo3::prelude::*;

#[pyclass(module = "docrpy", weakref)]
struct DocrEngine {
    #[pyo3(get)]
    language: String,
}

#[pymethods]
impl DocrEngine {
    #[new]
    fn new(lang_tag: String) -> PyResult<Self> {
        match docr::create_language_from_tag(&lang_tag) {
            Ok(_) => Ok(DocrEngine { language: lang_tag }),
            Err(e) => {
                let err = match e {
                    docr::RuntimeError(..) => OSError::py_err(e.to_string()),
                    docr::OperationError(..) => ValueError::py_err(e.to_string()),
                };
                Err(err)
            }
        }
    }

    #[staticmethod]
    fn get_supported_languages() -> PyResult<Vec<String>> {
        let result = docr::get_ocr_languages();
        if result.is_err() {
            Err(RuntimeError::py_err("Could not get  supported languages."))
        } else {
            Ok(result.unwrap())
        }
    }

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
