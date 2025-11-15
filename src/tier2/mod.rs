#[cfg(feature = "alloc")]
pub mod smith_predictor;

#[cfg(feature = "alloc")]
pub use smith_predictor::SmithPredictor;
#[cfg(feature = "alloc")]
pub use smith_predictor::SmithPredictorFiltered;
