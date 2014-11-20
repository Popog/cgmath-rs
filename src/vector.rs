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

//! Types and traits for two, three, and four-dimensional vectors.
//!
//! ## Working with Vectors
//!
//! Vectors can be created in several different ways. There is, of course, the
//! traditional `new()` method, but unit vectors, zero vectors, and an identity
//! vector are also provided:
//!
//! ```rust
//! use cgmath::{Vector2, Vector3, Vector4};
//!
//! assert_eq!(Vector2::new(1.0f64, 0.0f64), Vector2::unit_x());
//! assert_eq!(Vector3::new(0.0f64, 0.0f64, 0.0f64), Vector3::zero());
//! assert_eq!(Vector4::from_value(1.0f64), Vector4::ident());
//! ```
//!
//! Vectors can be manipulated with typical mathematical operations (addition,
//! subtraction, element-wise multiplication, element-wise division, negation)
//! using the built-in operators. The additive and multiplicative inverses
//! (zero and one) provided by the standard library's `Zero` and `One` are also
//! available:
//!
//! ```rust
//! use std::num::{Zero, One};
//! use cgmath::{Vector2, Vector3, Vector4};
//!
//! let a: Vector2<f64> = Vector2::new(3.0, 4.0);
//! let b: Vector2<f64> = Vector2::new(-3.0, -4.0);
//!
//! assert_eq!(a + b, Zero::zero());
//! assert_eq!(-(a * b), Vector2::new(9.0f64, 16.0f64));
//! assert_eq!(a / One::one(), a);
//!
//! // As with Rust's `int` and `f32` types, Vectors of different types cannot
//! // be added and so on with impunity. The following will fail to compile:
//! // let c = a + Vector3::new(1.0, 0.0, 2.0);
//!
//! // Instead, we need to convert the Vector2 to a Vector3 by "extending" it
//! // with the value for the last coordinate:
//! let c: Vector3<f64> = a.extend(0.0) + Vector3::new(1.0, 0.0, 2.0);
//!
//! // Similarly, we can "truncate" a Vector4 down to a Vector3:
//! let d: Vector3<f64> = c + Vector4::unit_x().truncate();
//!
//! assert_eq!(d, Vector3::new(5.0f64, 4.0f64, 2.0f64));
//! ```
//!
//! Vectors also provide methods for typical operations such as
//! [scalar multiplication](http://en.wikipedia.org/wiki/Scalar_multiplication),
//! [dot products](http://en.wikipedia.org/wiki/Dot_product),
//! and [cross products](http://en.wikipedia.org/wiki/Cross_product).
//!
//! ```rust
//! use std::num::Zero;
//! use cgmath::{Vector, Vector2, Vector3, Vector4, dot};
//!
//! // All vectors implement the dot product as a method:
//! let a: Vector2<f64> = Vector2::new(3.0, 6.0);
//! let b: Vector2<f64> = Vector2::new(-2.0, 1.0);
//! assert_eq!(a.dot(&b), Zero::zero());
//!
//! // But there is also a top-level function:
//! assert_eq!(a.dot(&b), dot(a, b));
//!
//! // Scalar multiplication can return a new object, or be done in place
//! // to avoid an allocation:
//! let mut c: Vector4<f64> = Vector4::from_value(3.0);
//! let d: Vector4<f64> = c.mul_s(2.0);
//! c.mul_self_s(2.0);
//! assert_eq!(c, d);
//!
//! // Cross products are defined for 3-dimensional vectors:
//! let e: Vector3<f64> = Vector3::unit_x();
//! let f: Vector3<f64> = Vector3::unit_y();
//! assert_eq!(e.cross(&f), Vector3::unit_z());
//! ```
//!
//! Several other useful methods are provided as well. Vector fields can be
//! accessed using array syntax (i.e. `vector[0] == vector.x`), or by using
//! the methods provided by the [`Array1`](../array/trait.Array1.html) trait.
//! This trait also provides a `map()` method for applying arbitrary functions.
//!
//! The [`Vector`](../trait.Vector.html) trait presents the most general
//! features of the vectors, while [`EuclideanVector`]
//! (../array/trait.EuclideanVector.html) is more specific to Euclidean space.

use std::fmt;
use std::mem;
use std::num::{Zero, zero, One, one};
use std::rand::{Rand, Rng};

use angle::{Rad, atan2, acos};
use approx::{ApproxEq, Epsilon, epsilon};
use array::{Array1, FixedArray};
use num::{BaseNum, BaseFloat};

/// A trait that specifies a range of numeric operations for vectors. Not all
/// of these make sense from a linear algebra point of view, but are included
/// for pragmatic reasons.
pub trait Vector<S: BaseNum>: Array1<S>
                  + Neg<Self>
                  + Zero
                  + One
                  + Rand {
    /// Add a scalar to this vector, returning a new vector.
    fn add_s(&self, s: S) -> Self;
    /// Subtract a scalar from this vector, returning a new vector.
    fn sub_s(&self, s: S) -> Self;
    /// Multiply this vector by a scalar, returning a new vector.
    fn mul_s(&self, s: S) -> Self;
    /// Divide this vector by a scalar, returning a new vector.
    fn div_s(&self, s: S) -> Self;
    /// Take the remainder of this vector by a scalar, returning a new vector.
    fn rem_s(&self, s: S) -> Self;

    /// Add this vector to another, returning a new vector.
    fn add_v(&self, v: &Self) -> Self;
    /// Subtract another vector from this one, returning a new vector.
    fn sub_v(&self, v: &Self) -> Self;
    /// Multiply this vector by another, returning a new vector.
    fn mul_v(&self, v: &Self) -> Self;
    /// Divide this vector by another, returning a new vector.
    fn div_v(&self, v: &Self) -> Self;
    /// Take the remainder of this vector by another, returning a new scalar.
    fn rem_v(&self, v: &Self) -> Self;

    /// Negate this vector in-place.
    fn neg_self(&mut self);

    /// Add a scalar to this vector in-place.
    fn add_self_s(&mut self, s: S);
    /// Subtract a scalar from this vector, in-place.
    fn sub_self_s(&mut self, s: S);
    /// Multiply this vector by a scalar, in-place.
    fn mul_self_s(&mut self, s: S);
    /// Divide this vector by a scalar, in-place.
    fn div_self_s(&mut self, s: S);
    /// Take the remainder of this vector by a scalar, in-place.
    fn rem_self_s(&mut self, s: S);

    /// Add another vector to this one, in-place.
    fn add_self_v(&mut self, v: &Self);
    /// Subtract another vector from this one, in-place.
    fn sub_self_v(&mut self, v: &Self);
    /// Multiply this matrix by another, in-place.
    fn mul_self_v(&mut self, v: &Self);
    /// Divide this matrix by anothor, in-place.
    fn div_self_v(&mut self, v: &Self);
    /// Take the remainder of this vector by another, in-place.
    fn rem_self_v(&mut self, v: &Self);

    /// The sum of each component of the vector.
    fn comp_add(&self) -> S;
    /// The product of each component of the vector.
    fn comp_mul(&self) -> S;

    /// Vector dot product.
    #[inline]
    fn dot(&self, other: &Self) -> S { self.mul_v(other).comp_add() }

    /// Returns the squared length of the vector. This does not perform an
    /// expensive square root operation like in the `length` method and can
    /// therefore be more efficient for comparing the lengths of two vectors.
    #[inline]
    fn length2(&self) -> S { self.dot(self) }

    /// with θ = the angle between `self` and `other`, returns `true` if abs(cos(θ)) <= epsilon
    #[inline]
    fn is_perpendicular_eps(&self, other: &Self, epsilon: &S) -> bool {
        let (a, b) = (self, other);
        // We're looking to return abs(cos(θ)) <= ε
        // Proof:
        // ∵             a·b = |a|*|b|*cos(θ)
        // ∵        |cos(θ)| <= ε
        //    |a|*|b|*cos(θ) <= |a|*|b|*ε
        // (|a|*|b|*cos(θ))² <= (|a|*|b|*ε)²
        // ∴          (a·b)² <= |a|²*|b|²*ε²
        let a_dot_b = a.dot(b);
        a_dot_b * a_dot_b <= a.length2() * b.length2() * (*epsilon) * (*epsilon)
    }

    /// with θ = the angle between `self` and `other`, returns `true` if abs(sin(θ)) <= epsilon
    #[inline]
    fn is_parallel_eps(&self, other: &Self, epsilon: &S) -> bool {
        let (a, b) = (self, other);
        // We're looking to return abs(sin(θ)) < ε
        // Proof:
        // ∵                         a·b = |a|*|b|*cos(θ)
        // ∵                     sin²(θ) = 1 - cos²(θ)
        // ∵                    |sin(θ)| <= ε
        //              |a|*|b|*|sin(θ)| <= |a|*|b|*ε
        //             (|a|*|b|*sin(θ))² <= (|a|*|b|*ε)²
        //             |a|²*|b|²*sin²(θ) <= |a|²*|b|²*ε²
        //       |a|²*|b|²*(1 - cos²(θ)) <= |a|²*|b|²*ε²
        // |a|²*|b|² - |a|²*|b|²*cos²(θ) <= |a|²*|b|²*ε²
        // |a|²*|b|² - (|a|*|b|*cos(θ))² <= |a|²*|b|²*ε²
        //            |a|²*|b|² - (a·b)² <= |a|²*|b|²*ε²
        //      |a|²*|b|² - |a|²*|b|²*ε² <= (a·b)²
        // ∴          |a|²*|b|²*(1 - ε²) <= (a·b)²
        //
        // Specialized alternatives exist for 2d and 3d, but I'm not sure they're more efficient or
        // accurate. Needs some testing.
        let a_dot_b = a.dot(b);
        let one: S = One::one();
        a.length2() * b.length2() * (one - (*epsilon) * (*epsilon)) <= a_dot_b * a_dot_b
    }

    /// The minimum component of the vector.
    fn comp_min(&self) -> S;
    /// The maximum component of the vector.
    fn comp_max(&self) -> S;
}

/// Dot product of two vectors.
#[inline] pub fn dot<S: BaseNum, V: Vector<S>>(a: V, b: V) -> S { a.dot(&b) }

// Utility macro for generating associated functions for the vectors
macro_rules! vec(
    ($Self:ident <$S:ident> { $($field:ident),+ }, $n:expr) => (
        #[deriving(PartialEq, Eq, Clone, Hash, Encodable, Decodable)]
        pub struct $Self<S> { $(pub $field: S),+ }

        impl<$S> $Self<$S> {
            /// Construct a new vector, using the provided values.
            #[inline]
            pub fn new($($field: $S),+) -> $Self<$S> {
                $Self { $($field: $field),+ }
            }
        }

        impl<$S: Copy> $Self<$S> {
            /// Construct a vector from a single value, replicating it.
            #[inline]
            pub fn from_value(value: $S) -> $Self<$S> {
                $Self { $($field: value),+ }
            }
        }

        impl<$S: BaseNum> $Self<$S> {
            /// The additive identity of the vector.
            #[inline]
            pub fn zero() -> $Self<$S> { $Self::from_value(zero()) }

            /// The multiplicative identity of the vector.
            #[inline]
            pub fn ident() -> $Self<$S> { $Self::from_value(one()) }
        }

        impl<$S> FixedArray<[$S, ..$n]> for $Self<$S> {
            #[inline]
            fn into_fixed(self) -> [$S, ..$n] {
                match self { $Self { $($field),+ } => [$($field),+] }
            }

            #[inline]
            fn as_fixed<'a>(&'a self) -> &'a [$S, ..$n] {
                unsafe { mem::transmute(self) }
            }

            #[inline]
            fn as_mut_fixed<'a>(&'a mut self) -> &'a mut [$S, ..$n] {
                unsafe { mem::transmute(self) }
            }

            #[inline]
            fn from_fixed(_v: [$S, ..$n]) -> $Self<$S> {
                // match v { [$($field),+] => $Self { $($field: $field),+ } }
                fail!("Unimplemented, pending a fix for rust-lang/rust#16418")
            }

            #[inline]
            fn from_fixed_ref<'a>(v: &'a [$S, ..$n]) -> &'a $Self<$S> {
                unsafe { mem::transmute(v) }
            }

            #[inline]
            fn from_fixed_mut<'a>(v: &'a mut [$S, ..$n]) -> &'a mut $Self<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<$S: Copy> Index<uint, S> for $Self<$S> {
            #[inline]
            fn index<'a>(&'a self, i: &uint) -> &'a $S {
                &self.as_fixed()[*i]
            }
        }

        impl<$S: Copy> IndexMut<uint, S> for $Self<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: &uint) -> &'a mut $S {
                &mut self.as_mut_fixed()[*i]
            }
        }

        impl<$S: Copy> Array1<$S> for $Self<$S> {
            #[inline]
            fn map(&mut self, op: |$S| -> $S) -> $Self<$S> {
                $(self.$field = op(self.$field);)+ *self
            }
        }

        impl<S: BaseNum> Vector<S> for $Self<S> {
            #[inline] fn add_s(&self, s: S) -> $Self<S> { $Self::new($(self.$field + s),+) }
            #[inline] fn sub_s(&self, s: S) -> $Self<S> { $Self::new($(self.$field - s),+) }
            #[inline] fn mul_s(&self, s: S) -> $Self<S> { $Self::new($(self.$field * s),+) }
            #[inline] fn div_s(&self, s: S) -> $Self<S> { $Self::new($(self.$field / s),+) }
            #[inline] fn rem_s(&self, s: S) -> $Self<S> { $Self::new($(self.$field % s),+) }

            #[inline] fn add_v(&self, v: &$Self<S>) -> $Self<S> { $Self::new($(self.$field + v.$field),+) }
            #[inline] fn sub_v(&self, v: &$Self<S>) -> $Self<S> { $Self::new($(self.$field - v.$field),+) }
            #[inline] fn mul_v(&self, v: &$Self<S>) -> $Self<S> { $Self::new($(self.$field * v.$field),+) }
            #[inline] fn div_v(&self, v: &$Self<S>) -> $Self<S> { $Self::new($(self.$field / v.$field),+) }
            #[inline] fn rem_v(&self, v: &$Self<S>) -> $Self<S> { $Self::new($(self.$field % v.$field),+) }

            #[inline] fn neg_self(&mut self) { $(self.$field = -self.$field;)+ }

            #[inline] fn add_self_s(&mut self, s: S) { $(self.$field = self.$field + s;)+ }
            #[inline] fn sub_self_s(&mut self, s: S) { $(self.$field = self.$field - s;)+ }
            #[inline] fn mul_self_s(&mut self, s: S) { $(self.$field = self.$field * s;)+ }
            #[inline] fn div_self_s(&mut self, s: S) { $(self.$field = self.$field / s;)+ }
            #[inline] fn rem_self_s(&mut self, s: S) { $(self.$field = self.$field % s;)+ }

            #[inline] fn add_self_v(&mut self, v: &$Self<S>) { $(self.$field = self.$field + v.$field;)+ }
            #[inline] fn sub_self_v(&mut self, v: &$Self<S>) { $(self.$field = self.$field - v.$field;)+ }
            #[inline] fn mul_self_v(&mut self, v: &$Self<S>) { $(self.$field = self.$field * v.$field;)+ }
            #[inline] fn div_self_v(&mut self, v: &$Self<S>) { $(self.$field = self.$field / v.$field;)+ }
            #[inline] fn rem_self_v(&mut self, v: &$Self<S>) { $(self.$field = self.$field % v.$field;)+ }

            #[inline] fn comp_add(&self) -> S { fold!(&add, { $(self.$field),+ }) }
            #[inline] fn comp_mul(&self) -> S { fold!(&mul, { $(self.$field),+ }) }
            #[inline] fn comp_min(&self) -> S { fold!(partial_min, { $(self.$field),+ }) }
            #[inline] fn comp_max(&self) -> S { fold!(partial_max, { $(self.$field),+ }) }
        }

        impl<S: BaseNum> Add<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn add(&self, v: &$Self<S>) -> $Self<S> { self.add_v(v) }
        }

        impl<S: BaseNum> Sub<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn sub(&self, v: &$Self<S>) -> $Self<S> { self.sub_v(v) }
        }

        impl<S: BaseNum> Zero for $Self<S> {
            #[inline] fn zero() -> $Self<S> { $Self::from_value(zero()) }
            #[inline] fn is_zero(&self) -> bool { *self == zero() }
        }

        impl<S: BaseNum> Neg<$Self<S>> for $Self<S> {
            #[inline] fn neg(&self) -> $Self<S> { $Self::new($(-self.$field),+) }
        }

        impl<S: BaseNum> Mul<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn mul(&self, v: &$Self<S>) -> $Self<S> { self.mul_v(v) }
        }

        impl<S: BaseNum> Div<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn div(&self, v: &$Self<S>) -> $Self<S> { self.div_v(v) }
        }

        impl<S: BaseNum> Rem<$Self<S>, $Self<S>> for $Self<S> {
            #[inline] fn rem(&self, v: &$Self<S>) -> $Self<S> { self.rem_v(v) }
        }

        impl<S: BaseNum> One for $Self<S> {
            #[inline] fn one() -> $Self<S> { $Self::from_value(one()) }
        }

        impl<S: BaseNum+Epsilon> ApproxEq<S> for $Self<S> {
            #[inline]
            fn approx_eq_eps(&self, other: &$Self<S>, epsilon: &S) -> bool {
                // Two vectors are approximately equal if the distance between them
                // is relatively small in comparison. This scale factor can be either the min
                // or the max of the two lengths (it doesn't make that much of a difference which),
                // but it should not be less than 1. A scale factor of less than 1 would prevent
                // zeros from being approximately equal to each other correctly.
                // We can not square root both sides to make stuff faster
                // Proof:
                // ∵ |a-b|  <= max(|a|,|b|,1) * ε
                //   |a-b|² <= max(|a|,|b|,1)² * ε²
                // ∴ |a-b|² <= max(|a|²,|b|²,1) * ε²
                let scale = self.length2().partial_max(other.length2());
                if scale > one() {
                    (*self - *other).length2() <= (*epsilon) * (*epsilon) * scale
                } else {
                    (*self - *other).length2() <= (*epsilon) * (*epsilon)
                }
            }
        }

        impl<S: Rand> Rand for $Self<S> {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $Self<S> {
                $Self { $($field: rng.gen::<S>()),+ }
            }
        }
    )
)

macro_rules! fold {
    (&$method:ident, { $x:expr, $y:expr })                   => { $x.$method(&$y) };
    (&$method:ident, { $x:expr, $y:expr, $z:expr })          => { $x.$method(&$y).$method(&$z) };
    (&$method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => { $x.$method(&$y).$method(&$z).$method(&$w) };
    ($method:ident, { $x:expr, $y:expr })                    => { $x.$method($y) };
    ($method:ident, { $x:expr, $y:expr, $z:expr })           => { $x.$method($y).$method($z) };
    ($method:ident, { $x:expr, $y:expr, $z:expr, $w:expr })  => { $x.$method($y).$method($z).$method($w) };
}

vec!(Vector2<S> { x, y }, 2)
vec!(Vector3<S> { x, y, z }, 3)
vec!(Vector4<S> { x, y, z, w }, 4)

/// Operations specific to numeric two-dimensional vectors.
impl<S: BaseNum> Vector2<S> {
    /// A unit vector in the `x` direction.
    #[inline] pub fn unit_x() -> Vector2<S> { Vector2::new(one(), zero()) }
    /// A unit vector in the `y` direction.
    #[inline] pub fn unit_y() -> Vector2<S> { Vector2::new(zero(), one()) }

    /// The perpendicular dot product of the vector and `other`.
    #[inline]
    pub fn perp_dot(&self, other: &Vector2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Create a `Vector3`, using the `x` and `y` values from this vector, and the
    /// provided `z`.
    #[inline]
    pub fn extend(&self, z: S)-> Vector3<S> {
        Vector3::new(self.x, self.y, z)
    }

    /// with θ = the angle between `self` and `other`, returns `true` if abs(sin(θ)) <= epsilon
    #[inline]
    pub fn is_parallel_eps(&self, other: &Vector2<S>, epsilon: &S) -> bool {
        let (a, b) = (self, other);
        // We're looking to return abs(sin(θ)) < ε
        // Proof:
        // ∵            a⊥·b = |a|*|b|*sin(θ)
        // ∵        |sin(θ)| <= ε
        //  |a|*|b|*|sin(θ)| <= |a|*|b|*ε
        // (|a|*|b|*sin(θ))² <= (|a|*|b|*ε)²
        // ∴         (a⊥·b)² <= |a|²*|b|²*ε²
        let a_perp_dot_b = a.perp_dot(b);
        a_perp_dot_b * a_perp_dot_b <= a.length2() * b.length2() * (*epsilon) * (*epsilon)
    }
}

/// Operations specific to numeric three-dimensional vectors.
impl<S: BaseNum> Vector3<S> {
    /// A unit vector in the `x` direction.
    #[inline] pub fn unit_x() -> Vector3<S> { Vector3::new(one(), zero(), zero()) }
    /// A unit vector in the `y` direction.
    #[inline] pub fn unit_y() -> Vector3<S> { Vector3::new(zero(), one(), zero()) }
    /// A unit vector in the `w` direction.
    #[inline] pub fn unit_z() -> Vector3<S> { Vector3::new(zero(), zero(), one()) }

    /// Returns the cross product of the vector and `other`.
    #[inline]
    pub fn cross(&self, other: &Vector3<S>) -> Vector3<S> {
        Vector3::new((self.y * other.z) - (self.z * other.y),
                     (self.z * other.x) - (self.x * other.z),
                     (self.x * other.y) - (self.y * other.x))
    }

    /// Calculates the cross product of the vector and `other`, then stores the
    /// result in `self`.
    #[inline]
    pub fn cross_self(&mut self, other: &Vector3<S>) {
        *self = self.cross(other)
    }

    /// Create a `Vector4`, using the `x`, `y` and `z` values from this vector, and the
    /// provided `w`.
    #[inline]
    pub fn extend(&self, w: S)-> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, w)
    }

    /// Create a `Vector2`, dropping the `z` value.
    #[inline]
    pub fn truncate(&self)-> Vector2<S> {
        Vector2::new(self.x, self.y)
    }

    /// with θ = the angle between `self` and `other`, returns `true` if abs(sin(θ)) <= epsilon
    #[inline]
    pub fn is_parallel_eps(&self, other: &Vector3<S>, epsilon: &S) -> bool {
        let (a, b) = (self, other);
        // We're looking to return abs(sin(θ)) < ε
        // Proof:
        // ∵           |a×b| = |a|*|b|*sin(θ)
        // ∵        |sin(θ)| <= ε
        //  |a|*|b|*|sin(θ)| <= |a|*|b|*ε
        // (|a|*|b|*sin(θ))² <= (|a|*|b|*ε)²
        // ∴          |a×b|² <= |a|²*|b|²*ε²
        a.cross(b).length2() <= (*epsilon) * (*epsilon) * a.length2() * b.length2()
    }
}

/// Operations specific to numeric four-dimensional vectors.
impl<S: BaseNum> Vector4<S> {
    /// A unit vector in the `x` direction.
    #[inline] pub fn unit_x() -> Vector4<S> { Vector4::new(one(), zero(), zero(), zero()) }
    /// A unit vector in the `y` direction.
    #[inline] pub fn unit_y() -> Vector4<S> { Vector4::new(zero(), one(), zero(), zero()) }
    /// A unit vector in the `z` direction.
    #[inline] pub fn unit_z() -> Vector4<S> { Vector4::new(zero(), zero(), one(), zero()) }
    /// A unit vector in the `w` direction.
    #[inline] pub fn unit_w() -> Vector4<S> { Vector4::new(zero(), zero(), zero(), one()) }

    /// Create a `Vector3`, dropping the `w` value.
    #[inline]
    pub fn truncate(&self)-> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }

    /// Create a `Vector3`, dropping the nth element
    #[inline]
    pub fn truncate_n(&self, n: int)-> Vector3<S> {
        match n {
            0 => Vector3::new(self.y, self.z, self.w),
            1 => Vector3::new(self.x, self.z, self.w),
            2 => Vector3::new(self.x, self.y, self.w),
            3 => Vector3::new(self.x, self.y, self.z),
            _ => fail!("{} is out of range", n)
        }
    }
}

/// Specifies geometric operations for vectors. This is only implemented for
/// 2-dimensional and 3-dimensional vectors.
pub trait EuclideanVector<S: BaseFloat>: Vector<S>
                                       + ApproxEq<S> {
    /// with θ = the angle between `self` and `other`, returns `true` if abs(cos(θ)) < epsilon()
    #[inline]
    fn is_perpendicular(&self, other: &Self) -> bool {
        self.is_perpendicular_eps(other, &epsilon())
    }

    /// with θ = the angle between `self` and `other`, returns `true` if abs(sin(θ)) < epsilon()
    #[inline]
    fn is_parallel(&self, other: &Self) -> bool {
        self.is_parallel_eps(other, &epsilon())
    }

    /// The norm of the vector.
    #[inline]
    fn length(&self) -> S {
        self.length2().sqrt()
    }

    /// The angle between the vector and `other`, in radians.
    fn angle(&self, other: &Self) -> Rad<S> {
        let (a, b) = (self, other);
        // We're looking to return θ
        // Proof:
        // ∵  |a|*|b|*cos(θ) = a·b
        //            cos(θ) = a·b / (|a|*|b|)
        //                 θ = acos(a·b / (|a|*|b|))
        // ∴               θ = acos(a·b * rsqrt(|a|²*|b|²))
        acos(a.dot(b) * (a.length2() * b.length2()).rsqrt())
    }

    /// Returns a vector with the same direction, but with a `length` (or
    /// `norm`) of `1`.
    #[inline]
    fn normalize(&self) -> Self {
        self.normalize_to(one::<S>())
    }

    /// Returns a vector with the same direction and a given `length`.
    #[inline]
    fn normalize_to(&self, length: S) -> Self {
        self.mul_s(length * self.length2().rsqrt())
    }

    /// Returns the result of linarly interpolating the length of the vector
    /// towards the length of `other` by the specified amount.
    #[inline]
    fn lerp(&self, other: &Self, amount: S) -> Self {
        self.add_v(&other.sub_v(self).mul_s(amount))
    }

    /// Normalises the vector to a length of `1`.
    #[inline]
    fn normalize_self(&mut self) {
        self.normalize_self_to(one::<S>())
    }

    /// Normalizes the vector to `length`.
    #[inline]
    fn normalize_self_to(&mut self, length: S) {
        let scale = length * self.length2().rsqrt();
        self.mul_self_s(scale);
    }

    /// Linearly interpolates the length of the vector towards the length of
    /// `other` by the specified amount.
    fn lerp_self(&mut self, other: &Self, amount: S) {
        let v = other.sub_v(self).mul_s(amount);
        self.add_self_v(&v);
    }
}

impl<S: BaseFloat> EuclideanVector<S> for Vector2<S> {
    #[inline]
    fn angle(&self, other: &Vector2<S>) -> Rad<S> {
        let (a, b) = (self, other);
        // We're looking to return θ
        // Proof:
        // ∵            a⊥·b = |a|*|b|*sin(θ)
        // ∵             a·b = |a|*|b|*cos(θ)
        // ∵          tan(θ) = sin(θ) / cos(θ)
        //            tan(θ) = (|a|*|b|*sin(θ)) / (|a|*|b|*cos(θ))
        //            tan(θ) = a⊥·b / a·b
        // ∴               θ = atan2(|a⊥·b|, a·b)
        atan2(a.perp_dot(b), a.dot(b))
    }
}

impl<S: BaseFloat> EuclideanVector<S> for Vector3<S> {
    #[inline]
    fn angle(&self, other: &Vector3<S>) -> Rad<S> {
        let (a, b) = (self, other);
        // We're looking to return θ
        // Proof:
        // ∵           |a×b| = |a|*|b|*sin(θ)
        // ∵             a·b = |a|*|b|*cos(θ)
        // ∵          tan(θ) = sin(θ) / cos(θ)
        //            tan(θ) = (|a|*|b|*sin(θ)) / (|a|*|b|*cos(θ))
        //            tan(θ) = |a×b| / a·b
        // ∴               θ = atan2(|a×b|, a·b)
        atan2(a.cross(b).length(), a.dot(b))
    }
}

impl<S: BaseFloat> EuclideanVector<S> for Vector4<S> {
}

impl<S: BaseNum> fmt::Show for Vector2<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<S: BaseNum> fmt::Show for Vector3<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<S: BaseNum> fmt::Show for Vector4<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}
