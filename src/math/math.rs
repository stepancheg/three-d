pub(crate) use cgmath::ortho;
pub(crate) use cgmath::perspective;
#[doc(hidden)]
pub use cgmath::prelude::*;
use cgmath::{Deg, Matrix2, Matrix3, Matrix4, Point3, Rad, Vector2, Vector3, Vector4};
use std::alloc::Layout;
use std::slice;

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
pub type Mat2 = Matrix2<f32>;
pub type Mat3 = Matrix3<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Point = Point3<f32>;
pub type Degrees = Deg<f32>;
pub type Radians = Rad<f32>;

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vector2::new(x, y)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vector3::new(x, y, z)
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vector4::new(x, y, z, w)
}

pub fn vector3_vec_flatten<T: Copy>(vec: Vec<Vector3<T>>) -> Vec<T> {
    // TODO: this can be transmute
    let mut r = Vec::with_capacity(vec.len() * 3);
    for v in &vec {
        r.push(v.x);
        r.push(v.y);
        r.push(v.z);
    }
    r
}

pub fn vector3_slice_flatten<T>(slice: &[Vector3<T>]) -> &[T] {
    // this should be no-op, because `Vector3` is `#[repr(C)]`
    assert_eq!(Layout::array::<T>(3).unwrap(), Layout::new::<Vector3<T>>());

    unsafe { slice::from_raw_parts(slice.as_ptr() as *const T, slice.len() * 3) }
}

pub fn vector3_vec_fold<T: Copy>(vec: Vec<T>) -> Vec<Vector3<T>> {
    // TODO: this can be transmute
    let mut r = Vec::with_capacity(vec.len() / 3);
    for i in 0..vec.len() / 3 {
        r.push(Vector3::new(vec[i * 3], vec[i * 3 + 1], vec[i * 3 + 2]))
    }
    r
}

pub(crate) trait Vec2Ext {
    fn to_slice(&self) -> [f32; 2];
}

impl Vec2Ext for Vec2 {
    fn to_slice(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

pub(crate) trait Vec3Ext {
    fn to_slice(&self) -> [f32; 3];
}

impl Vec3Ext for Vec3 {
    fn to_slice(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

pub(crate) trait Vec4Ext {
    fn to_slice(&self) -> [f32; 4];
}

impl Vec4Ext for Vec4 {
    fn to_slice(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

pub(crate) trait Mat2Ext {
    fn to_slice(&self) -> [f32; 4];
}

impl Mat2Ext for Mat2 {
    fn to_slice(&self) -> [f32; 4] {
        [self.x.x, self.x.y, self.y.x, self.y.y]
    }
}

pub(crate) trait Mat3Ext {
    fn to_slice(&self) -> [f32; 9];
}

impl Mat3Ext for Mat3 {
    fn to_slice(&self) -> [f32; 9] {
        [
            self.x.x, self.x.y, self.x.z, self.y.x, self.y.y, self.y.z, self.z.x, self.z.y,
            self.z.z,
        ]
    }
}

pub(crate) trait Mat4Ext {
    fn to_slice(&self) -> [f32; 16];
}

impl Mat4Ext for Mat4 {
    fn to_slice(&self) -> [f32; 16] {
        [
            self.x.x, self.x.y, self.x.z, self.x.w, self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w, self.w.x, self.w.y, self.w.z, self.w.w,
        ]
    }
}

pub fn degrees(v: f32) -> Degrees {
    Deg(v)
}
pub fn radians(v: f32) -> Radians {
    Rad(v)
}

pub fn rotation_matrix_from_dir_to_dir(source_dir: Vec3, target_dir: Vec3) -> Mat4 {
    let c = source_dir.dot(target_dir);
    if c > 0.99999 {
        return Mat4::identity();
    }
    if c < -0.99999 {
        return Mat4::from_scale(-1.0);
    }
    let axis = source_dir.cross(target_dir).normalize();

    let s = (1.0 - c * c).sqrt();
    let oc = 1.0 - c;
    return Mat4::new(
        oc * axis.x * axis.x + c,
        oc * axis.x * axis.y - axis.z * s,
        oc * axis.z * axis.x + axis.y * s,
        0.0,
        oc * axis.x * axis.y + axis.z * s,
        oc * axis.y * axis.y + c,
        oc * axis.y * axis.z - axis.x * s,
        0.0,
        oc * axis.z * axis.x - axis.y * s,
        oc * axis.y * axis.z + axis.x * s,
        oc * axis.z * axis.z + c,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
    .transpose();
}

#[cfg(test)]
mod test {
    use crate::vec3;
    use crate::vector3_slice_flatten;
    use crate::vector3_vec_flatten;
    use crate::vector3_vec_fold;

    #[test]
    fn test_vector3_vec_flatten() {
        assert_eq!(
            vec![1.0, 1.1, 1.2, 2.0, 2.1, 2.2],
            vector3_vec_flatten(vec![vec3(1.0, 1.1, 1.2), vec3(2.0, 2.1, 2.2)])
        );
    }

    #[test]
    fn test_vector3_slice_flatten() {
        assert_eq!(
            &[1.0, 1.1, 1.2, 2.0, 2.1, 2.2],
            vector3_slice_flatten(&[vec3(1.0, 1.1, 1.2), vec3(2.0, 2.1, 2.2)])
        );
    }

    #[test]
    fn test_vector3_vec_fold() {
        assert_eq!(
            vec![vec3(1.0, 1.1, 1.2), vec3(2.0, 2.1, 2.2)],
            vector3_vec_fold(vec![1.0, 1.1, 1.2, 2.0, 2.1, 2.2])
        );
    }
}
