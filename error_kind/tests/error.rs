#![allow(dead_code)]
extern crate error_kind;

use error_kind::error_kind_for;

#[error_kind_for(FooError)]
pub enum FooErrorKind {
    Bar,
    Baz,
}

#[test]
fn new_method_works() {
    let err = FooError::new(FooErrorKind::Bar);
    assert_eq!(err.kind(), FooErrorKind::Bar);
    assert!(err.source().is_none());
}

#[test]
fn with_source_method_works() {
    let source = std::io::Error::new(std::io::ErrorKind::Other, "other");
    let err = FooError::with_source(FooErrorKind::Baz, source);

    assert_eq!(err.kind(), FooErrorKind::Baz);
    assert_eq!(err.source().unwrap().to_string(), "other");
}

#[test]
fn format_source_method_works_with_source() {
    let source = std::io::Error::new(std::io::ErrorKind::Other, "other");
    let err = FooError::with_source(FooErrorKind::Baz, source);

    assert_eq!(err.format_source(), "other");
}

#[test]
fn format_source_method_works_without_source() {
    let err = FooError::new(FooErrorKind::Baz);
    assert_eq!(err.format_source(), "unknown");
}

#[test]
fn kind_method_works() {
    let err = FooError::new(FooErrorKind::Bar);
    let kind = err.kind();
    let _kind = err.kind();

    assert_eq!(kind, FooErrorKind::Bar);
}

#[test]
fn error_implements_debug() {
    let err = FooError::new(FooErrorKind::Bar);
    let _ = format!("{:?}", err);
    let _: Box<dyn Debug> = Box::new(err);
}

#[test]
fn error_implements_display() {
    let err = FooError::new(FooErrorKind::Bar);
    let _ = format!("{}", err);
    let _: Box<dyn Display> = Box::new(err);
}

#[test]
fn error_implements_error() {
    let err = FooError::new(FooErrorKind::Bar);
    let _: Box<dyn Error> = Box::new(err);
}
