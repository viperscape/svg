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

extern crate svg;

use std::io::{BufWriter};
use std::collections::HashMap;
use std::fs::OpenOptions;

use svg::SVG;
use svg::Transform;
// use svg::{Circle, Rect, RoundedRect};

pub fn main() {
    let mut output = BufWriter::new(
        OpenOptions::new().write(true).create(true).truncate(true).open("output.svg").unwrap()
    );
    let mut image = SVG::new(12, 12);
    let mut attribs = HashMap::new();
    let polygon_points: Vec<(i32, i32)> = vec![
            (350,75),  (379,161), (469,161), (397,215),
            (423,301), (350,250), (277,301), (303,215),
            (231,161), (321,161)];
    attribs.insert("fill".to_string(), "green".to_string());
    attribs.insert("stroke".to_string(), "orange".to_string());
    attribs.insert("stroke-width".to_string(), "2".to_string());

    let mut t = Transform::new();
    t.translate(100, 200);
    t.translate(10, 32);
    println!("{}", t.get());

    image.view_box(0, 0, 1200, 400);
    image.g_begin(Some("First_Group"), Some(&t), Some(&attribs));
    image.g_begin(Some("First_Group"), Some(&t), Some(&attribs));
    // image.g_transform(t.clone());
    //image.g_rotate(15);
    image.circle(600, 200, 100, "id=jojo fill=red stroke=blue stroke-width=10");
    image.rect(700, 200, 200, 200, "fill=red stroke=blue stroke-width=10");
    image.rounded_rect(800, 600, 200, 200, 60, 30, "fill=red stroke=blue stroke-width=10");
    image.polygon(&polygon_points, "fill=red stroke=blue stroke-width=10");
    image.text(0,0,"this is a\ntest!","font-family=Verdana font-size=55 fill=blue");
    image.g_end();
    image.g_end();
    image.title("Svg library test Main!");
    image.desc("A simple main test for the rust svg generation library");

    match image.finalize(&mut output) {
        Ok(_)    => {},
        Err(err) => panic!("{}", err)
    }
}
