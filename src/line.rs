// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directionectory of this distribution.
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

//! Line segments

use num::{BaseNum};
use point::{Point, Point2, Point3};
use vector::{Vector};

/// A generic directed line segment from `origin` to `dest`.
#[deriving(Clone, PartialEq, Encodable, Decodable)]
pub struct Line<P> {
    pub origin: P,
    pub dest: P,
}

impl<S: BaseNum, V: Vector<S>, P: Point<S, V>>  Line<P> {
    pub fn new(origin: P, dest: P) -> Line<P> {
        Line { origin:origin, dest:dest }
    }
}

pub type Line2<S> = Line<Point2<S>>;
pub type Line3<S> = Line<Point3<S>>;
