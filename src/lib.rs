extern crate num;
extern crate xml;

pub use svg::{SVG,SVGEntity};
pub use shapes::{Circle, Rect, RoundedRect, Ellipse, Line, PolyLine, Polygon};
pub use common::{rgb, rgba};
pub use text::Text;
pub use transform::Transform;

mod shapes;
mod text;
mod transform;
mod common;
pub mod svg;


use xml::{Parser, ElementBuilder, Element};
use std::fs::{File};
use std::io::Read;

#[test]
fn test_xml () {
    let mut p = xml::Parser::new();
    let mut e = xml::ElementBuilder::new();

    p.feed_str("<a href='//example.com'/>");
    for elem in p.filter_map(|x| e.handle_event(x)) {
        assert!(elem.is_ok());
    }
}

#[test]
fn test_xml_svg () {
    let mut p = xml::Parser::new();
    let mut eb = xml::ElementBuilder::new();

    let mut f = match File::open("assets/rust-logo-blk.svg") {
        Ok(file) => file,
        Err(err) => { println!("fail: load file");
                      panic!(err) },
    };
    
    let mut c = String::new();
    if let Err(err) = f.read_to_string(&mut c) {
        println!("fail: read file");
        panic!(err);
    };
    
    p.feed_str(&c);
    for e in p.filter_map(|x| eb.handle_event(x)) {
        assert!(e.is_ok());
    }
}

#[test]
fn test_svg () {
    let v = load_svg("assets/rust-logo-blk.svg");
    let svg = build_svg(&v[0]);
    assert_eq!(svg.head.dim.0,144);
}


fn load_svg (path: &str) -> Vec<Element> {
    let mut p = xml::Parser::new();
    let mut eb = xml::ElementBuilder::new();

    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(err) => { println!("fail: load file");
                      panic!(err) },
    };
    
    let mut c = String::new();
    if let Err(err) = f.read_to_string(&mut c) {
        println!("fail: read file");
        panic!(err);
    };
    
    p.feed_str(&c);
    let mut ve = vec!();
    for e in p.filter_map(|x| eb.handle_event(x)) {
        if let Ok(_e) = e {
            ve.push(_e);
        }
    }

    ve
}

fn build_svg (e: &Element) -> SVG {
    let mut image = SVG::new(100,100);

    // build view box for svg
    if let Some(vb) = parse_viewbox(e).ok() {
        image.view_box(vb.0,vb.1,vb.2,vb.3);
    }

    // set xy
    if let Some(xy) = parse_xy(e).ok() {
        image.head.xy = xy;
    }

    // set dimensions
    if let Some(d) = parse_dim(e).ok() {
        image.head.dim = d;
    }

    image
}

fn parse_viewbox (e: &Element) -> Result<(i32,i32,i32,i32),&str> {
    if let Some(_vb) = e.attributes.get(&("viewBox".to_string(), None)) {
        let mut vb = vec!();
        for i in _vb.split(' ') {
            match i.parse::<i32>() {
                Ok(v) => vb.push(v),
                Err(e) => return Err("ParseIntErr, malformed viewbox"),
            }
        }
        if vb.len() > 3 { return Ok((vb[0], vb[1],vb[2],vb[3])) }
    }

    Err("No viewbox")
}

fn parse_px<'e> (e: &'e Element, s: &str) -> Result<i32,&'e str> {
    if let Some(i) = e.attributes.get(&(s.to_string(), None)) {
        let is: Vec<&str> = i.split("px").collect();
        match is[0].parse::<i32>() {
            Ok(v) => return Ok(v),
            _ => (),
        }
    }
    
    Err("No px")
}

fn parse_dim (e: &Element) -> Result<(i32,i32),&str> {
    if let Some(w) = parse_px(e,"width").ok() {
        if let Some(h) = parse_px(e,"height").ok() {
            return Ok((w,h))
        }
    }

    Err("No dimensions")
}

fn parse_xy (e: &Element) -> Result<(i32,i32),&str> {
    if let Some(x) = parse_px(e,"x").ok() {
        if let Some(y) = parse_px(e,"y").ok() {
            return Ok((x,y))
        }
    }

    Err("No XY")
}

fn parse_title (e: &Element) -> Result<&str,&str> {
    if let Some(t) = e.attributes.get(&("title".to_string(), None)) {
        return Ok(t)
    }

    Err("No title")
}

fn parse_desc (e: &Element) -> Result<&str,&str> {
    if let Some(t) = e.attributes.get(&("desc".to_string(), None)) {
        return Ok(t)
    }

    Err("No description")
}


/*enum ParseErr {
    Malformed(String),
    Missing(String),
}*/
