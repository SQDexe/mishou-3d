/*! Custom error representations and failure handling mechanisms. */

/** Module containing error definitions related to the application's core lifecycle phases. */
mod app_error;
/** Module containing miscellaneous error definitions, such as those for model loading and image resizing. */
mod other_error;
/** Module containing error definitions related to parsing operations, such as interpreting arrays and colours. */
mod parser_error;



pub use app_error::AppCreateError;
pub use app_error::AppResumeError;
pub use app_error::AppRuntimeError;
pub use other_error::LoadingInProgressError;
pub use other_error::DepthImageResizeError;
pub use other_error::LoadModelError;
// pub use parser_error::NotFiniteError;
pub use parser_error::ParseArrayError;
pub use parser_error::ParseColorError;
// pub use parser_error::ParseFiniteFloatError;
// pub use parser_error::ParseRotationVectorError;
// pub use parser_error::ParseVectorError;