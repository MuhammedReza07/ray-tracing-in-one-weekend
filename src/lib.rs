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

/// Linear algebra functions for vectors in 3-dimensional Euclidean space (i.e. `R^3`).
pub mod vector3;

/// The functions in the `vector3` module, now using SIMD via the x86/x86_64 SIMD extensions provided in SSE(<=4.1).
/// 
/// Note that unlike `vector3::Vector3`, each component of the vectors in this library are `f32`, rather than `f64`.
/// Also, there is a fourth component, `w`, which is why the name includes `vector4` instead of `vector3`.
pub mod vector4;