//! The `DefmtFormatTrait` trait is used to mark types that can be formatted for `defmt`.
//! This is used to conditionally require that `defmt::Format` is implemented for a type if the `defmt` feature is enabled.

#[cfg(feature = "defmt")]
pub trait DefmtFormatTrait: defmt::Format {}

#[cfg(not(feature = "defmt"))]
pub trait DefmtFormatTrait {}
