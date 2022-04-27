use nalgebra_glm as glm;
use glm::{DVec3, U32Vec3};
use super::utils::*;

use std::hash::BuildHasherDefault;
use hashbrown::HashMap;
use std::cmp::Ordering;

pub mod group;
pub use group::Group as Group; 

#[cfg(feature = "interop")]
pub mod unsafe_mesh;
#[cfg(feature = "interop")]
pub use unsafe_mesh::UnsafeMesh as UnsafeMesh; 

pub mod shared_mesh;
pub use shared_mesh::SharedMesh as SharedMesh; 

include!("connected_mesh.rs");
include!("builders.rs");