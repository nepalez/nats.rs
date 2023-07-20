#![allow(dead_code)]
extern crate error_kind;

use error_kind::error_kind_for;

#[error_kind_for(FooError)]
pub enum FooErrorKind {
    Bar,
    Baz,
}

#[test]
fn error_kind_implements_clone() {
    let _ = FooErrorKind::Bar.clone();
}

#[test]
fn error_kind_implements_partial_eq() {
    let _: Box<dyn PartialEq<FooErrorKind>> = Box::new(FooErrorKind::Bar);

    assert_eq!(FooErrorKind::Bar, FooErrorKind::Bar);
    assert_ne!(FooErrorKind::Bar, FooErrorKind::Baz);
}

#[test]
fn error_kind_implements_debug() {
    let _: Box<dyn Debug> = Box::new(FooErrorKind::Bar);
    let _ = format!("{:?}", FooErrorKind::Bar);
}
