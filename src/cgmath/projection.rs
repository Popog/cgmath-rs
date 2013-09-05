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

use std::num::{zero, one};

use angle::{Angle, rad, tan, cot};
use matrix::{Mat4, ToMat4};
use util::two;

/// Create a perspective projection matrix.
///
/// This is the equivalent to the [gluPerspective]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/gluPerspective.xml) function.
pub fn perspective<S: Clone + Float, A: Angle<S>>(fovy: A, aspect: S, near: S, far: S) -> Mat4<S> {
    PerspectiveFov {
        fovy:   fovy,
        aspect: aspect,
        near:   near,
        far:    far,
    }.to_mat4()
}

/// Create a perspective matrix from a view frustrum.
///
/// This is the equivalent of the now deprecated [glFrustrum]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml) function.
pub fn frustum<S: Clone + Float>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Mat4<S> {
    Perspective {
        left:   left,
        right:  right,
        bottom: bottom,
        top:    top,
        near:   near,
        far:    far,
    }.to_mat4()
}

/// Create an orthographic projection matrix.
///
/// This is the equivalent of the now deprecated [glOrtho]
/// (http://www.opengl.org/sdk/docs/man2/xhtml/glOrtho.xml) function.
pub fn ortho<S: Clone + Float>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Mat4<S> {
    Ortho {
        left:   left,
        right:  right,
        bottom: bottom,
        top:    top,
        near:   near,
        far:    far,
    }.to_mat4()
}

pub trait Projection<S>: ToMat4<S> {}

/// A perspective projection based on a vertical field-of-view angle.
#[deriving(Clone, Eq)]
pub struct PerspectiveFov<S, A> {
    fovy:   A,
    aspect: S,
    near:   S,
    far:    S,
}

impl<S: Clone + Float, A: Angle<S>> PerspectiveFov<S, A> {
    pub fn to_perspective(&self) -> Perspective<S> {
        let angle = self.fovy.div_s(two::<S>());
        let ymax = self.near * tan(angle);
        let xmax = ymax * self.aspect;

        Perspective {
            left:   -xmax,
            right:   xmax,
            bottom: -ymax,
            top:     ymax,
            near:    self.near.clone(),
            far:     self.far.clone(),
        }
    }
}

impl<S: Clone + Float, A: Angle<S>> ToMat4<S> for PerspectiveFov<S, A> {
    fn to_mat4(&self) -> Mat4<S> {
        let half_turn: A = Angle::from(rad::<S>(Real::frac_pi_2()));

        assert!(self.fovy   < zero(),    "The vertical field of view cannot be below zero, found: %?", self.fovy);
        assert!(self.fovy   > half_turn, "The vertical field of view cannot be greater than a half turn, found: %?", self.fovy);
        assert!(self.aspect < zero(),    "The aspect ratio cannot be below zero, found: %?", self.aspect);
        assert!(self.near   < zero(),    "The near plane distance cannot be below zero, found: %?", self.near);
        assert!(self.far    < zero(),    "The far plane distance cannot be below zero, found: %?", self.far);
        assert!(self.far    < self.near, "The far plane cannot be closer than the near plane, found: far: %?, near: %?", self.far, self.near);

        let f = cot(self.fovy.div_s(two::<S>()));

        let c0r0 = f / self.aspect;
        let c0r1 = zero();
        let c0r2 = zero();
        let c0r3 = zero();

        let c1r0 = zero();
        let c1r1 = f;
        let c1r2 = zero();
        let c1r3 = zero();

        let c2r0 = zero();
        let c2r1 = zero();
        let c2r2 = (self.far + self.near) / (self.near - self.far);
        let c2r3 = -one::<S>();

        let c3r0 = zero();
        let c3r1 = zero();
        let c3r2 = (two::<S>() * self.far * self.near) / (self.near - self.far);
        let c3r3 = zero();

        Mat4::new(c0r0, c0r1, c0r2, c0r3,
                  c1r0, c1r1, c1r2, c1r3,
                  c2r0, c2r1, c2r2, c2r3,
                  c3r0, c3r1, c3r2, c3r3)
    }
}

impl<S: Clone + Float, A: Angle<S>> Projection<S> for PerspectiveFov<S, A>;

/// A perspective projection with arbitrary left/right/bottom/top distances
#[deriving(Clone, Eq)]
pub struct Perspective<S> {
    left:   S,  right:  S,
    bottom: S,  top:    S,
    near:   S,  far:    S,
}

impl<S: Clone + Float> ToMat4<S> for Perspective<S> {
    fn to_mat4(&self) -> Mat4<S> {
        assert!(self.left   > self.right, "`left` cannot be greater than `right`, found: left: %? right: %?", self.left, self.right);
        assert!(self.bottom > self.top,   "`bottom` cannot be greater than `top`, found: bottom: %? top: %?", self.bottom, self.top);
        assert!(self.near   > self.far,   "`near` cannot be greater than `far`, found: near: %? far: %?", self.near, self.far);

        let c0r0 = (two::<S>() * self.near) / (self.right - self.left);
        let c0r1 = zero();
        let c0r2 = zero();
        let c0r3 = zero();

        let c1r0 = zero();
        let c1r1 = (two::<S>() * self.near) / (self.top - self.bottom);
        let c1r2 = zero();
        let c1r3 = zero();

        let c2r0 = (self.right + self.left) / (self.right - self.left);
        let c2r1 = (self.top + self.bottom) / (self.top - self.bottom);
        let c2r2 = -(self.far + self.near) / (self.far - self.near);
        let c2r3 = -one::<S>();

        let c3r0 = zero();
        let c3r1 = zero();
        let c3r2 = -(two::<S>() * self.far * self.near) / (self.far - self.near);
        let c3r3 = zero();

        Mat4::new(c0r0, c0r1, c0r2, c0r3,
                  c1r0, c1r1, c1r2, c1r3,
                  c2r0, c2r1, c2r2, c2r3,
                  c3r0, c3r1, c3r2, c3r3)
    }
}

impl<S: Clone + Float> Projection<S> for Perspective<S>;

/// An orthographic projection with arbitrary left/right/bottom/top distances
#[deriving(Clone, Eq)]
pub struct Ortho<S> {
    left:   S,  right:  S,
    bottom: S,  top:    S,
    near:   S,  far:    S,
}

impl<S: Clone + Float> ToMat4<S> for Ortho<S> {
    fn to_mat4(&self) -> Mat4<S> {
        assert!(self.left   > self.right, "`left` cannot be greater than `right`, found: left: %? right: %?", self.left, self.right);
        assert!(self.bottom > self.top,   "`bottom` cannot be greater than `top`, found: bottom: %? top: %?", self.bottom, self.top);
        assert!(self.near   > self.far,   "`near` cannot be greater than `far`, found: near: %? far: %?", self.near, self.far);

        let c0r0 = two::<S>() / (self.right - self.left);
        let c0r1 = zero();
        let c0r2 = zero();
        let c0r3 = zero();

        let c1r0 = zero();
        let c1r1 = two::<S>() / (self.top - self.bottom);
        let c1r2 = zero();
        let c1r3 = zero();

        let c2r0 = zero();
        let c2r1 = zero();
        let c2r2 = -two::<S>() / (self.far - self.near);
        let c2r3 = -one::<S>();

        let c3r0 = -(self.right + self.left) / (self.right - self.left);
        let c3r1 = -(self.top + self.bottom) / (self.top - self.bottom);
        let c3r2 = -(self.far + self.near) / (self.far - self.near);
        let c3r3 = one::<S>();

        Mat4::new(c0r0, c0r1, c0r2, c0r3,
                  c1r0, c1r1, c1r2, c1r3,
                  c2r0, c2r1, c2r2, c2r3,
                  c3r0, c3r1, c3r2, c3r3)
    }
}

impl<S: Clone + Float> Projection<S> for Ortho<S>;