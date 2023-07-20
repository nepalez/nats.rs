//! This is a procedural macro crate that implements `ErrorKind` derive macro
//! for building an error type from a enum, representing possible kinds of an error.
//! 
//! ## Simple Example
//! 
//! ```
//! use error_kind::error_kind_for;
//! 
//! // This generates the `FooError { kind, source }` error type, that implements Error trait.
//! // The kind of the error will be of the `FooErrorKind` type.
//! #[error_kind_for(FooError)]
//! pub enum FooErrorKind {
//!   Bar,
//!   Baz,
//! }
//! 
//! let error = FooError::new(FooErrorKind::Bar);
//! assert_eq!(error.kind(), FooErrorKind::Bar);
//! ```
//! 
//! ## Use the `with_source` constructor:
//! 
//! ```
//! # use error_kind::error_kind_for;
//! # #[error_kind_for(FooError)]
//! # pub enum FooErrorKind {
//! #   Bar,
//! #   Baz,
//! # }
//!
//! let source = std::io::Error::new(std::io::ErrorKind::Other, "other");
//! let error = FooError::with_source(FooErrorKind::Baz, source);
//! assert_eq!(error.kind(), FooErrorKind::Baz);
//! assert_eq!(error.source().unwrap().to_string(), "other");
//! ```
//! 
//! Notice: for the simplicity, this crate presumes the existence of `crate::Error` type.

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Ident, ItemEnum};

#[proc_macro_attribute]
pub fn error_kind_for(args: TokenStream, kind: TokenStream) -> TokenStream {
    let kind_definition = parse_macro_input!(kind as ItemEnum);
    let kind: &Ident = &kind_definition.ident;
    let err = parse_macro_input!(args as syn::Ident);
    
    let result = quote! {
        use std::error::Error;
        use std::fmt::{self, Debug, Display, Formatter};

        #[derive(Clone, Debug, PartialEq)]
        #kind_definition

        #[derive(Debug)]
        pub struct #err {
            kind: #kind,
            source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
        }

        impl #err {
            pub(crate) fn new(kind: #kind) -> Self {
                Self { kind, source: None }
            }
            
            pub(crate) fn with_source<S>(kind: #kind, source: S) -> Self
            where
                S: Error + Send + Sync + 'static,
            {
                Self {
                    kind,
                    source: Some(source.into()),
                }
            }
        
            pub fn kind(&self) -> #kind {
                // Almost all `kind` types implement `Copy`, so it's almost always copy.
                // We need to clone, as some more complex one may have nested other errors, that
                // implement Clone only.
                self.kind.clone()
            }
        
            pub(crate) fn format_source(&self) -> String {
                self.source
                    .as_ref()
                    .map(|err| err.to_string())
                    .unwrap_or("unknown".to_string())
            }
        }
        
        impl Display for #err {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        impl std::error::Error for #err {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.source.as_ref().map(|err| err.as_ref() as _)
            }
        }
    };
    
    result.into()
}
