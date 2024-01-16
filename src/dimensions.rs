use cgmath::{Vector3, Zero};

/// A simple AABB box to determine the dimensions of a node.
/// 
/// From should be the smallest point. IE (-1, -1, -1).
/// To should be the largest point.  IE (1, 1, 1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub from: Vector3<f32>,
    pub to: Vector3<f32>
}

/// Add a default for dimensions to support Node's default.
impl Default for Dimensions {
    fn default() -> Self {
        Self {
            from: Vector3::zero(),
            to: Vector3::zero()
        }
    }
}

impl Dimensions {
    /// Checks if two dimensions overlap.
    /// 
    /// Arguments:
    /// * &self - Our dimension
    /// * other: &Dimensions - Other dimension to compare against.
    /// 
    /// Returns true of the two dimensions overlap.  False if they do not.
    pub fn overlap(&self, other: &Dimensions) -> bool {
        return self.from.x < other.to.x && other.from.x < self.to.x && 
                self.from.y < other.to.y && other.from.y < self.to.y &&
                self.from.z < other.to.z && other.from.z < self.to.z
    }
}