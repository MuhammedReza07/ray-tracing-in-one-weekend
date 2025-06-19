/// Abstractions for working with the camera and scene-rendering.
pub mod camera;

/// Functions for working with RGB colours as real vectors with components in the range `[0, 1]`.
pub mod color;

/// Abstractions for working with materials and various instances of materials.
pub mod materials;

/// Abstractions for working with orientable objects, i.e. objects that admit an assignment of
/// surface normals to each point of their surface.
pub mod orientable;

/// Abstractions for working with "intersectable" objects, e.g. surfaces.
pub mod intersectable;

/// Various random number, or rather vector, generation utilities.
pub mod random;

/// Abstractions for working with rays.
pub mod ray;

/// Naive collection for ray tracing of multi-object scenes.
pub mod renderable_list;

/// Intersectable surfaces.
pub mod surfaces;

/// Linear algebra functions for vectors in 4-dimensional Euclidean space (i.e. `R^4`), including some functions 
/// for vectors in 3-dimensional space which may be applied to 4-vectors with `w = 0`.
/// 
/// The functions in this module use intrinsics for Intel's SSE ISA extensions to improve performance by leveraging SIMD,
/// but they have fallback variants for older (<4.1) SSE versions and no SSE at all.
pub mod vector4;