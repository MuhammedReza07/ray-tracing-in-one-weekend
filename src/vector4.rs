mod base;

mod sse;

#[cfg(not(target_feature = "sse"))]
pub type Vector4 = base::Vector4;

#[cfg(target_feature = "sse")]
pub type Vector4 = sse::Vector4;