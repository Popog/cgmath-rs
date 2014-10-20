// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::one;
use std::fmt;

use approx::ApproxEq;
use num::BaseFloat;
use point::{Point, Point3};
use vector::{EuclideanVector, Vector, Vector3, Vector4};


/// A 3-dimensional plane formed from the equation: `A*x + B*y + C*z - D = 0`.
///
/// # Fields
///
/// - `n`: a unit vector representing the normal of the plane where:
///   - `n.x`: corresponds to `A` in the plane equation
///   - `n.y`: corresponds to `B` in the plane equation
///   - `n.z`: corresponds to `C` in the plane equation
/// - `d`: the distance value, corresponding to `D` in the plane equation
///
/// # Notes
///
/// The `A*x + B*y + C*z - D = 0` form is preferred over the other common
/// alternative, `A*x + B*y + C*z + D = 0`, because it tends to avoid
/// superfluous negations (see _Real Time Collision Detection_, p. 55).
#[deriving(Clone, PartialEq, Encodable, Decodable)]
pub struct Plane<S> {
    pub n: Vector3<S>,
    pub d: S,
}

impl<S: BaseFloat> Plane<S> {
    /// Construct a plane from a normal vector and a scalar distance. The
    /// plane will be perpendicular to `n`, and `d` units offset from the
    /// origin.
    pub fn new(n: Vector3<S>, d: S) -> Plane<S> {
        Plane { n: n, d: d }
    }

    /// # Arguments
    ///
    /// - `a`: the `x` component of the normal
    /// - `b`: the `y` component of the normal
    /// - `c`: the `z` component of the normal
    /// - `d`: the plane's distance value
    pub fn from_abcd(a: S, b: S, c: S, d: S) -> Plane<S> {
        Plane { n: Vector3::new(a, b, c), d: d }
    }

    /// Construct a plane from the components of a four-dimensional vector
    pub fn from_vector4(v: Vector4<S>) -> Plane<S> {
        match v {
            Vector4 { x, y, z, w } => Plane { n: Vector3::new(x, y, z), d: w },
        }
    }

    /// Constructs a plane that passes through the the three points `a`, `b` and `c`
    pub fn from_points(a: Point3<S>, b: Point3<S>, c: Point3<S>) -> Option<Plane<S>> {
        // create two vectors that run parallel to the plane
        let v0 = b.sub_p(&a);
        let v1 = c.sub_p(&a);

        // find the normal vector that is perpendicular to v1 and v2
        let n = v0.cross(&v1);

        if n.approx_eq(&Vector3::zero()) { None }
        else {
            // compute the distance to the plane
            let d = -a.dot(&n);

            Some(Plane::new(n, d))
        }
    }

    /// Construct a plane from a point and a normal vector.
    /// The plane will contain the point `p` and be perpendicular to `n`.
    pub fn from_point_normal(p: Point3<S>, n: Vector3<S>) -> Plane<S> {
        Plane { n: n, d: p.dot(&n) }
    }

	/// Returns an approximately equal plane, but with `n` having a `length` (or
	/// `norm`) of `1`.
	#[inline]
	pub fn normalize_normal(&self) -> Plane<S> {
		self.normalize_normal_to(one::<S>())
	}

	/// Returns an approximately equal plane, but with `n` having a given `length`.
	#[inline]
	pub fn normalize_normal_to(&self, length: S) -> Plane<S> {
		let scale = length / self.n.length();
		Plane::new(self.n.mul_s(scale), self.d * scale)
	}

	/// Normalizes the plane normal to a length of `1`.
	#[inline]
	pub fn normalize_normal_self(&mut self) {
		self.normalize_normal_self_to(one::<S>())
	}

	/// Normalizes the plane normal to `length`.
	#[inline]
	pub fn normalize_normal_self_to(&mut self, length: S) {
		let scale = length / self.n.length();
		self.n.mul_self_s(scale);
		self.d = self.d * scale;
	}

	/// Returns an approximately equal plane, but with `d` = `1`.
	#[inline]
	pub fn normalize_distance(&self) -> Plane<S> {
		self.normalize_distance_to(one::<S>())
	}

	/// Returns an approximately equal plane, but with `d` = `length`.
	#[inline]
	pub fn normalize_distance_to(&self, length: S) -> Plane<S> {
		let scale = self.d / length;
		Plane::new(self.n.mul_s(scale), length)
	}

	/// Normalizes the plane normal such that `d` = `1`.
	#[inline]
	pub fn normalize_distance_self(&mut self) {
		self.normalize_distance_self_to(one::<S>())
	}

	/// Normalizes the plane normal such that `d` = `length`.
	#[inline]
	pub fn normalize_distance_self_to(&mut self, length: S) {
		let scale = self.d / length;
		self.n.mul_self_s(scale);
		self.d = length;
	}
}

impl<S: BaseFloat + ApproxEq<S>>
ApproxEq<S> for Plane<S> {
    #[inline]
    fn approx_eq_eps(&self, other: &Plane<S>, epsilon: &S) -> bool {
        self.n.mul_s(self.d).approx_eq_eps(&other.n.mul_s(other.d), epsilon)
    }
}

impl<S: BaseFloat> fmt::Show for Plane<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:f}x + {:f}y + {:f}z - {:f} = 0",
               self.n.x, self.n.y, self.n.z, self.d)
    }
}
