// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

use std::num;

pub trait Epsilon {
	// FIXME (#5527): This should be an associated constant
	#[inline]
	fn epsilon() -> Self;
}

/// Returns the epsilon value for a given type.
#[inline(always)] pub fn epsilon<T: Epsilon>() -> T { Epsilon::epsilon() }

pub trait ApproxEq<T: Epsilon> {
	#[inline]
	fn approx_eq(&self, other: &Self) -> bool {
		let eps: T = epsilon();
		self.approx_eq_eps(other, &eps)
	}

	fn approx_eq_eps(&self, other: &Self, epsilon: &T) -> bool;
}

macro_rules! approx_float_impl(
	($t:ty $v:expr) => (
		impl Epsilon for $t {
			#[inline]
			fn epsilon() -> $t { $v }
		}
		impl ApproxEq<$t> for $t {
			 #[inline]
			fn approx_eq_eps(&self, other: &$t, epsilon: &$t) -> bool {
				num::abs(*self - *other) < *epsilon
			}
		}
	)
)

approx_float_impl!(f32 1.0e-5f32)
approx_float_impl!(f64 1.0e-5f64)

