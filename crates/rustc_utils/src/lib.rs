//! `rustc-utils` provides a wide variety of utilities for working with the Rust compiler.
//! We developed these functions in the course of building various research projects with
//! rustc.
//!
//! Most of the functionality is organized into extension traits implemented for types
//! in the compiler, such as one for MIR control-flow graphs ([`BodyExt`]) or one for
//! text ranges ([`SpanExt`]).
//!
//! This crate is pinned to a specific nightly version of the Rust compiler.
//! See the [`rustc-plugin` README](https://github.com/cognitive-engineering-lab/rustc-plugin)
//! for details on how to add `rustc-utils` as a dependency.

#![feature(
  rustc_private,
  negative_impls,        // for !Send
  min_specialization,    // for rustc_index::newtype_index 
  type_alias_impl_trait, // for iterators in traits
  lazy_cell,             // for global constants w/ heap allocation
  box_patterns,          // for ergonomics
  let_chains,            // for places_conflict module
  exact_size_is_empty,   // for graphviz module
)]
#![allow(clippy::len_zero)]

extern crate either;
extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_graphviz;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_macros;
#[macro_use]
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_mir_transform;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate smallvec;

pub mod cache;
pub mod hir;
pub mod mir;
pub mod source_map;
#[cfg(feature = "test")]
pub mod test_utils;
pub mod timer;

pub use crate::{
  hir::{region::RegionExt, ty::TyExt},
  mir::{
    adt_def::AdtDefExt, body::BodyExt, mutability::MutabilityExt, operand::OperandExt,
    place::PlaceExt,
  },
  source_map::span::{SpanDataExt, SpanExt},
};
