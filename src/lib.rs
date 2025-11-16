//! This crate contains a collection of tools to generate 3D models
//! ([OpenSCAD](https://www.openscad.org/) description programs) representing profile data
//! from GitHub.

/// Librariy with tools to generate 3D models
/// ([OpenSCAD](https://www.openscad.org/) description programs) representing data
/// from GitHub user.
pub mod generators;
/// Module to download daily activity from GitHub user
/// profles in a given date range.
pub mod github;
pub mod openscad;


