use crate::{math::lerp, vector::Vector3};

#[test]
fn lerp_test() {
    assert_eq!(lerp(&Vector3(1.0, 1.0, 1.0), &Vector3(3.0, 3.0, 3.0), 0.5), Vector3(2.0, 2.0, 2.0))
}