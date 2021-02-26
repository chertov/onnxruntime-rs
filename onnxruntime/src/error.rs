//! Module containing error definitions.

use std::{io, path::PathBuf};

use thiserror::Error;

use onnxruntime_sys as sys;

use crate::{char_p_to_string};

/// Type alias for the `Result`
pub type Result<T> = std::result::Result<T, OrtError>;

/// Error type centralizing all possible errors
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum OrtError {
    /// The C API can message to the caller using a C `char *` which needs to be converted
    /// to Rust's `String`. This operation can fail.
    #[error("Failed to construct String")]
    StringConversion(OrtApiError),
    // FIXME: Move these to another enum (they are C API calls errors)
    /// An error occurred when creating an ONNX environment
    #[error("Failed to create environment: {0}")]
    Environment(OrtApiError),
    /// Error occurred when creating an ONNX session options
    #[error("Failed to create session options: {0}")]
    SessionOptions(OrtApiError),
    /// Error occurred when creating an ONNX session
    #[error("Failed to create session: {0}")]
    Session(OrtApiError),
    /// Error occurred when creating an ONNX allocator
    #[error("Failed to get allocator: {0}")]
    Allocator(OrtApiError),
    /// Error occurred when counting ONNX input or output count
    #[error("Failed to get input or output count: {0}")]
    InOutCount(OrtApiError),
    /// Error occurred when getting ONNX input name
    #[error("Failed to get input name: {0}")]
    InputName(OrtApiError),
    /// Error occurred when getting ONNX type information
    #[error("Failed to get type info: {0}")]
    GetTypeInfo(OrtApiError),
    /// Error occurred when casting ONNX type information to tensor information
    #[error("Failed to cast type info to tensor info: {0}")]
    CastTypeInfoToTensorInfo(OrtApiError),
    /// Error occurred when getting tensor elements type
    #[error("Failed to get tensor element type: {0}")]
    TensorElementType(OrtApiError),
    /// Error occurred when getting ONNX dimensions count
    #[error("Failed to get dimensions count: {0}")]
    GetDimensionsCount(OrtApiError),
    /// Error occurred when getting ONNX dimensions
    #[error("Failed to get dimensions: {0}")]
    GetDimensions(OrtApiError),
    /// Error occurred when creating CPU memory information
    #[error("Failed to get dimensions: {0}")]
    CreateCpuMemoryInfo(OrtApiError),
    /// Error occurred when creating ONNX tensor
    #[error("Failed to create tensor: {0}")]
    CreateTensor(OrtApiError),
    /// Error occurred when creating ONNX tensor with specific data
    #[error("Failed to create tensor with data: {0}")]
    CreateTensorWithData(OrtApiError),
    /// Error occurred when filling a tensor with string data
    #[error("Failed to fill string tensor: {0}")]
    FillStringTensor(OrtApiError),
    /// Error occurred when checking if ONNX tensor was properly initialized
    #[error("Failed to check if tensor: {0}")]
    IsTensor(OrtApiError),
    /// Error occurred when getting tensor type and shape
    #[error("Failed to get tensor type and shape: {0}")]
    GetTensorTypeAndShape(OrtApiError),
    /// Error occurred when ONNX inference operation was called
    #[error("Failed to run: {0}")]
    Run(OrtApiError),
    /// Error occurred when extracting data from an ONNX tensor into an C array to be used as an `ndarray::ArrayView`
    #[error("Failed to get tensor data: {0}")]
    GetTensorMutableData(OrtApiError),

    /// Error occurred when downloading a pre-trained ONNX model from the [ONNX Model Zoo](https://github.com/onnx/models)
    #[error("Failed to download ONNX model: {0}")]
    DownloadError(#[from] OrtDownloadError),

    /// Dimensions of input data and ONNX model loaded from file do not match
    #[error("Dimensions do not match: {0:?}")]
    NonMatchingDimensions(NonMatchingDimensionsError),
    /// File does not exists
    #[error("File {filename:?} does not exists")]
    FileDoesNotExists {
        /// Path which does not exists
        filename: PathBuf,
    },
    /// Path is an invalid UTF-8
    #[error("Path {path:?} cannot be converted to UTF-8")]
    NonUtf8Path {
        /// Path with invalid UTF-8
        path: PathBuf,
    },
    /// Attempt to build a Rust `CString` from a null pointer
    #[error("Failed to build CString when original contains null: {0}")]
    CStringNulError(#[from] std::ffi::NulError),

    #[cfg(feature = "dynamic-loading")]
    /// An error occurred when creating an ONNX environment
    #[error("Failed to load a dynamic library of ONNX runtime '{path:?}' err: {err:?}")]
    DynamicLibraryLoadingError{
        /// Library path which does not exists
        path: PathBuf,
        err: String,
    },
}

/// Error used when dimensions of input (from model and from inference call)
/// do not match (as they should).
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum NonMatchingDimensionsError {
    /// Number of inputs from model does not match number of inputs from inference call
    #[error("Non-matching number of inputs: {inference_input_count:?} for input vs {model_input_count:?} for model (inputs: {inference_input:?}, model: {model_input:?})")]
    InputsCount {
        /// Number of input dimensions used by inference call
        inference_input_count: usize,
        /// Number of input dimensions defined in model
        model_input_count: usize,
        /// Input dimensions used by inference call
        inference_input: Vec<Vec<usize>>,
        /// Input dimensions defined in model
        model_input: Vec<Vec<Option<u32>>>,
    },
}

/// Error details when ONNX C API fail
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum OrtApiError {
    /// Details as reported by the ONNX C API in case of error
    #[error("Error calling ONNX Runtime C function: {0}")]
    Msg(String),
    /// Details as reported by the ONNX C API in case of error cannot be converted to UTF-8
    #[error("Error calling ONNX Runtime C function and failed to convert error message to UTF-8")]
    IntoStringError(std::ffi::IntoStringError),
}

/// Error from downloading pre-trained model from the [ONNX Model Zoo](https://github.com/onnx/models).
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum OrtDownloadError {
    /// Generic input/output error
    #[error("Error downloading data to file: {0}")]
    IoError(#[from] io::Error),
    /// Error getting content-length from an HTTP GET request
    #[error("Error getting content-length")]
    ContentLengthError,
    /// Mismatch between amount of downloaded and expected bytes
    #[error("Error copying data to file: expected {expected} length, received {io}")]
    CopyError {
        /// Expected amount of bytes to download
        expected: u64,
        /// Number of bytes read from network and written to file
        io: u64,
    },
}

/// Wrapper type around a ONNX C API's `OrtStatus` pointer
///
/// This wrapper exists to facilitate conversion from C raw pointers to Rust error types
pub struct OrtStatusWrapper{
    status: *const sys::OrtStatus,
    api: sys::OrtApi,
}
impl OrtStatusWrapper {
    fn new(api: &sys::OrtApi, status: *const sys::OrtStatus) -> Self {
        Self{api: api.clone(), status}
    }
}
// impl From<*const sys::OrtStatus> for OrtStatusWrapper {
//     fn from(api: sys::OrtApi, status: *const sys::OrtStatus) -> Self {
//         OrtStatusWrapper{api, status}
//     }
// }

impl From<OrtStatusWrapper> for std::result::Result<(), OrtApiError> {
    fn from(ort_status: OrtStatusWrapper) -> Self {
        if ort_status.status.is_null() {
            Ok(())
        } else {
            let raw: *const i8 = unsafe { ort_status.api.GetErrorMessage.unwrap()(ort_status.status) };
            match char_p_to_string(raw) {
                Ok(msg) => Err(OrtApiError::Msg(msg)),
                Err(err) => match err {
                    OrtError::StringConversion(OrtApiError::IntoStringError(e)) => {
                        Err(OrtApiError::IntoStringError(e))
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
}

pub(crate) fn status_to_result(
    api: &sys::OrtApi,
    status: *const sys::OrtStatus,
) -> std::result::Result<(), OrtApiError> {
    let status_wrapper: OrtStatusWrapper = OrtStatusWrapper::new(api, status);
    status_wrapper.into()
}

/// A wrapper around a function on OrtApi that maps the status code into [OrtApiError]
pub(crate) unsafe fn call_ort<F>(api: &sys::OrtApi, mut f: F) -> std::result::Result<(), OrtApiError>
where
    F: FnMut(&sys::OrtApi) -> *const sys::OrtStatus,
{
    status_to_result(api, f(api))
}
