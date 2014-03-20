// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::fmt::Show;
use collections::HashMap;
use std::vec::Vec;

use common::{insert_attribs, insert_transform, finalize};
use transform::Transform;
use SVGEntity;

#[deriving(Show, Eq, Clone)]
pub struct Circle {
    x: i32,
    y: i32,
    radius: u32,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

#[deriving(Show, Eq, Clone)]
pub struct Ellipse {
    x: i32,
    y: i32,
    x_radius: u32,
    y_radius: u32,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

#[deriving(Show, Eq, Clone)]
pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

#[deriving(Show, Eq, Clone)]
pub struct RoundedRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    x_round: u32,
    y_round: u32,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

#[deriving(Show, Eq, Clone)]
pub struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

#[deriving(Show, Eq, Clone)]
pub struct PolyLine<T> {
    points: Vec<(T, T)>,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

#[deriving(Show, Eq, Clone)]
pub struct Polygon<T> {
    points: Vec<(T, T)>,
    attribs: HashMap<~str, ~str>,
    transform: Option<Transform>
}

impl<T: Num + Show> PolyLine<T> {
    pub fn add_point(&mut self, x: T, y: T) {
        self.points.push((x, y))
    }
}

impl<T: Num + Show> Polygon<T> {
    pub fn add_point(&mut self, x: T, y: T) {
        self.points.push((x, y))
    }
}

fn get_points<T: Num + Show>(points: &Vec<(T, T)>) -> ~str {
    let mut p: ~str = ~"points=\"";
    for &(ref x, ref y) in points.iter() {
        p.push_str(format!("{},{} ", x, y))
    }
    p.push_str("\"");
    p
}

impl SVGEntity for Circle {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\"",
                           self.x, self.y, self.radius));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}

impl<T: Num + Show> SVGEntity for PolyLine<T> {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<polyline {}", get_points(&self.points)));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}

impl<T: Num + Show> SVGEntity for Polygon<T> {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<polygon {}", get_points(&self.points)));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}

impl SVGEntity for Line {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"",
                           self.x1, self.y1, self.x2, self.y2));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}

impl SVGEntity for Ellipse {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"",
                           self.x, self.y, self.x_radius, self.y_radius));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}

impl SVGEntity for Rect {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
                           self.x, self.y, self.width, self.height));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}

impl SVGEntity for RoundedRect {
    fn gen_output(&self) -> ~str {
        let mut o = ~"";
        o.push_str(format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" \
                           rx=\"{}\" ry=\"{}\"",
                           self.x, self.y, self.width, self.height,
                           self.x_round, self.y_round));
        o = insert_attribs(insert_transform(o, &self.transform), &self.attribs);
        finalize(o)
    }
}
