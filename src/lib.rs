/// Functions for working with RGB colours as real vectors with components in the range `[0, 1]`.
pub mod color;

/// Abstractions for working with orientable objects, i.e. objects that admit an assignment of
/// surface normals to each point of their surface.
pub mod orientable;

/// Abstractions for working with "intersectable" objects, e.g. surfaces.
pub mod intersectable;

/// Naive collection for ray tracing of multi-object scenes.
pub mod renderable_list;

/// Abstractions for working with rays.
pub mod ray;

/// Intersectable surfaces.
pub mod surfaces;

/// Linear algebra functions for vectors in 3-dimensional Euclidean space (i.e. `R^3`).
pub mod vector3;