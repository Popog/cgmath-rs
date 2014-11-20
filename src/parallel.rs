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

use approx::{Epsilon, epsilon};
use line::{Line2, Line3};
use num::BaseFloat;
use plane::Plane;
use point::Point;
use ray::{Ray2, Ray3};
use vector::{Vector, Vector2, Vector3, Vector4};


/// Given two directions separated by an angle θ, will return if the abs(sin(θ)) < epsilon
pub trait Parallel<T: Epsilon> {
	#[inline]
	fn is_parallel(&self) -> bool {
		let eps: T = epsilon();
		self.is_parallel_eps(&eps)
	}
	fn is_parallel_eps(&self, epsilon: &T) -> bool;
}



impl<S: BaseFloat> Parallel<S> for (Vector2<S>, Vector2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref b) = *self;
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Vector2<S>, Line2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref l1) = *self;
		let b = l1.dest.sub_p(&l1.origin);
		a.is_parallel_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Vector2<S>, Ray2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref r1) = *self;
		let ref b = r1.direction;
		a.is_parallel_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Line2<S>, Vector2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref b) = *self;
		let a = l0.dest.sub_p(&l0.origin);
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Line2<S>, Line2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref l1) = *self;
		let (a, b) = (l0.dest.sub_p(&l0.origin), l1.dest.sub_p(&l1.origin));
		a.is_parallel_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Line2<S>, Ray2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref r1) = *self;
		let (a, ref b) = (l0.dest.sub_p(&l0.origin), r1.direction);
		a.is_parallel_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Ray2<S>, Vector2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref b) = *self;
		let ref a = r0.direction;
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Ray2<S>, Line2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref l1) = *self;
		let (ref a, b) = (r0.direction, l1.dest.sub_p(&l1.origin));
		a.is_parallel_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Ray2<S>, Ray2<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref r1) = *self;
		let (ref a, ref b) = (r0.direction, r1.direction);
		a.is_parallel_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Vector3<S>, Vector3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref b) = *self;
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Vector3<S>, Line3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref l1) = *self;
		let b = l1.dest.sub_p(&l1.origin);
		a.is_parallel_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Vector3<S>, Ray3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref r1) = *self;
		let ref b = r1.direction;
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Vector3<S>, Plane<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref p1) = *self;
		let ref b = p1.n;
		a.is_perpendicular_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Line3<S>, Vector3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref b) = *self;
		let a = l0.dest.sub_p(&l0.origin);
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Line3<S>, Line3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref l1) = *self;
		let (a, b) = (l0.dest.sub_p(&l0.origin), l1.dest.sub_p(&l1.origin));
		a.is_parallel_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Line3<S>, Ray3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref r1) = *self;
		let (a, ref b) = (l0.dest.sub_p(&l0.origin), r1.direction);
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Line3<S>, Plane<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref l0, ref p1) = *self;
		let (a, ref b) = (l0.dest.sub_p(&l0.origin), p1.n);
		a.is_perpendicular_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Ray3<S>, Vector3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref b) = *self;
		let ref a = r0.direction;
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Ray3<S>, Line3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref l1) = *self;
		let (ref a, b) = (r0.direction, l1.dest.sub_p(&l1.origin));
		a.is_parallel_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Ray3<S>, Ray3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref r1) = *self;
		let (ref a, ref b) = (r0.direction, r1.direction);
		a.is_parallel_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Ray3<S>, Plane<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref r0, ref p1) = *self;
		let (ref a, ref b) = (r0.direction, p1.n);
		a.is_perpendicular_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Plane<S>, Vector3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref p0, ref b) = *self;
		let ref a = p0.n;
		a.is_perpendicular_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Plane<S>, Line3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref p0, ref l1) = *self;
		let (ref a, b) = (p0.n, l1.dest.sub_p(&l1.origin));
		a.is_perpendicular_eps(&b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Plane<S>, Ray3<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref p0, ref r1) = *self;
		let (ref a, ref b) = (p0.n, r1.direction);
		a.is_perpendicular_eps(b, epsilon)
	}
}

impl<S: BaseFloat> Parallel<S> for (Plane<S>, Plane<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref p0, ref p1) = *self;
		let (ref a, ref b) = (p0.n, p1.n);
		a.is_parallel_eps(b, epsilon)
	}
}



impl<S: BaseFloat> Parallel<S> for (Vector4<S>, Vector4<S>) {
	#[inline]
	fn is_parallel_eps(&self, epsilon: &S) -> bool {
		let (ref a, ref b) = *self;
		a.is_parallel_eps(b, epsilon)
	}
}
