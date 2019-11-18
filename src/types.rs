/// Information about how a backend stores metadata about
/// given SQL types.
pub trait TypeMetadata {
    /// The actual type used to represent metadata.
    type TypeMetadata;
}

/// Indicates that a SQL type exists for a backend and defines
/// useful metadata for the backend.
pub trait HasSqlType<A>: TypeMetadata {
    fn metadata() -> Self::TypeMetadata;
}
