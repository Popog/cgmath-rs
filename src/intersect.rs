// Copyright 2013 The CGMath Developers. For a full listing of the authors,
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

use std::num::{Zero, zero, One, one};

use aabb::{Aabb2};
use line::{Line2};
use num::{BaseFloat};
use plane::Plane;
use point::{Point, Point2, Point3};
use ray::{Ray2, Ray3};
use sphere::Sphere;
use vector::{Vector, Vector2};

pub trait Intersect<Result> {
	fn intersection(&self) -> Result;
}

pub trait IntersectPoint<Result> {
	fn intersection_point(&self) -> Result;
}

impl<S: BaseFloat> Intersect<Option<S>> for (Ray2<S>, Aabb2<S>) {
	fn intersection(&self) -> Option<S> {
		let (ref ray, ref aabb) = *self;

		let mut tmin: S = Float::neg_infinity();
		let mut tmax: S = Float::infinity();

		if ray.direction.x != zero() {
			let tx1 = (aabb.min.x - ray.origin.x) / ray.direction.x;
			let tx2 = (aabb.max.x - ray.origin.x) / ray.direction.x;
			tmin = tmin.max(tx1.min(tx2));
			tmax = tmax.min(tx1.max(tx2));
		}

		if ray.direction.y != zero() {
			let ty1 = (aabb.min.y - ray.origin.y) / ray.direction.y;
			let ty2 = (aabb.max.y - ray.origin.y) / ray.direction.y;
			tmin = tmin.max(ty1.min(ty2));
			tmax = tmax.min(ty1.max(ty2));
		}

		if tmin < zero() && tmax < zero() {
			None
		}
		else if tmax >= tmin {
			if tmin >= zero() {
				Some(tmin)
			}
			else {
				Some(tmax)
			}
		}
		else {
			None
		}
	}
}

impl<S: BaseFloat> IntersectPoint<Option<Point2<S>>> for (Ray2<S>, Aabb2<S>) {
	fn intersection_point(&self) -> Option<Point2<S>> {
		let (ref ray, _) = *self;
		match self.intersection() {
			Some(t) => Some(Point2::new(ray.origin.x + ray.direction.x * t,
			                            ray.origin.y + ray.direction.y * t)),
			_ => None,
		}
	}
}

/// Determines if an intersection between a ray and a line segments is found.
impl<S: BaseFloat> IntersectPoint<Option<Point2<S>>> for (Ray2<S>, Line2<S>) {
    fn intersection_point(&self) -> Option<Point2<S>> {
		let (ref ray, ref line) = *self;

		let p = ray.origin;
		let q = line.origin;
		let r = ray.direction;
		let s = Vector2::new(line.dest.x - line.origin.x, line.dest.y - line.origin.y);
		let zero: S = Zero::zero();

		let cross_1 = r.perp_dot(&s);
		let qmp = Vector2::new(q.x - p.x, q.y - p.y);
		let cross_2 = qmp.perp_dot(&r);

		if cross_1 == zero {
			if cross_2 != zero {
				// parallel
				return None;
			}

			// collinear
			let q2mp = Vector2::new(line.dest.x - p.x, line.dest.y - p.y);
			let dot_1 = qmp.dot(&r);
			let dot_2 = q2mp.dot(&r);
			if (dot_1 <= zero && dot_2 >= zero) || (dot_1 >= zero && dot_2 <= zero) {
				return Some(p);
			}
			else if dot_1 >= zero && dot_2 >= zero {
				if dot_1 <= dot_2 {
					return Some(q);
				}
				else {
					return Some(line.dest);
				}
			}

			// no overlap exists
			return None;
		}

		let t = qmp.perp_dot(&s) / cross_1;
		let u = cross_2 / cross_1;

		if zero <= t && u >= zero && u <= One::one() {
			return Some(Point2::new(p.x + t*r.x, p.y + t*r.y));
		}

		return None;
    }
}

impl<S: BaseFloat> Intersect<Option<S>> for (Plane<S>, Ray3<S>) {
	fn intersection(&self) -> Option<S> {
		let (ref p, ref r) = *self;
		let t = -(p.d + r.origin.dot(&p.n)) / r.direction.dot(&p.n);
		if t < Zero::zero() { None }
		else { Some(t) }
	}
}

impl<S: BaseFloat> IntersectPoint<Option<Point3<S>>> for (Plane<S>, Ray3<S>) {
	fn intersection_point(&self) -> Option<Point3<S>> {
		let (_, ref r) = *self;
		match self.intersection() {
			Some(t) => Some(r.origin.add_v(&r.direction.mul_s(t))),
			_ => None,
		}
	}
}

impl<S: BaseFloat> Intersect<Option<Ray3<S>>> for (Plane<S>, Plane<S>) {
    fn intersection(&self) -> Option<Ray3<S>> {
        fail!("Not yet implemented");
    }
}

impl<S: BaseFloat> IntersectPoint<Option<Point3<S>>> for (Plane<S>, Plane<S>, Plane<S>) {
    fn intersection_point(&self) -> Option<Point3<S>> {
        fail!("Not yet implemented");
    }
}

impl<S: BaseFloat> IntersectPoint<Option<Point3<S>>> for (Sphere<S>, Ray3<S>) {
	fn intersection_point(&self) -> Option<Point3<S>> {
		let (ref s, ref r) = *self;
		let l = s.center.sub_p(&r.origin);
		let tca = l.dot(&r.direction);
		if tca < zero() { return None; }
		let d2 = l.dot(&l) - tca*tca;
		if d2 > s.radius*s.radius { return None; }
		let thc = (s.radius*s.radius - d2).sqrt();
		Some(r.origin.add_v(&r.direction.mul_s(tca - thc)))
	}
}
