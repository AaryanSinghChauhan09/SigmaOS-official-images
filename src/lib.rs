//! SigmaOS official-images component.
//!
//! Rust implementation of `archlinux/official-images`: Arch's fork of
//! docker-library/official-images used to generate automated PRs for
//! Arch Linux container images.
//!
//! Upstream defines Dockerfile templates per image (`archlinux`,
//! `base-devel`, etc.) with template variables. This port implements the
//! template model: variables, per-image Dockerfile rendering, and the
//! tag/release plan the automation produces.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod image;
pub mod template;

pub use error::ImageError;
pub use image::{ArchImage, ImageVariant, official_images, release_tags};
pub use template::{TemplateContext, render_template};
