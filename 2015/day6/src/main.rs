#[macro_use]
extern crate nom;

use nom::{IResult, GetOutput, digit, multispace};
use nom::IResult::*;

use std::str;
use std::str::FromStr;

#[derive(Debug)]
enum LightChangeTo {
    On,
    Off,
    Toggle
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Debug)]
struct Area(Point, Point);

#[derive(Debug)]
struct LightAction {
    change: LightChangeTo,
    area: Area
}

named!(light_change<&[u8], LightChangeTo>, alt!( map!(tag!("turn on"), |_| LightChangeTo::On) | map!(tag!("turn off"), |_| LightChangeTo::Off) | map!(tag!("toggle"), |_| LightChangeTo::Toggle) ) );

named!(strnum<&[u8], u32>, map_res!( map_res!(digit, str::from_utf8), FromStr::from_str) );

named!(point<&[u8], Point>, chain!( cx: strnum ~ tag!(",") ~ cy: strnum, || Point { x: cx, y: cy } ) );

named!(area<&[u8], Area>, chain!( p1: delimited!(opt!(multispace), point, opt!(multispace)) ~ tag!("through") ~ p2: delimited!(opt!(multispace), point, opt!(multispace)), || Area(p1, p2) ) );

named!(light_action<&[u8], LightAction>, chain!(ch: light_change ~ ar: area, || LightAction { change: ch, area: ar } ) );

fn main() {
    println!("{:?}", light_action(b"turn on 0,0 through 999,999"));
    let input = include_str!("input.txt");
    //let input = "toggle 0,0 through 999,999";
    let actions: Vec<LightAction> = input.lines().map(|x| {
        match light_action(x.as_bytes()) {
            Done(_, o) => o,
            _ => unimplemented!()
        }
    }).collect();
    let mut lights = [[false; 1000]; 1000];
    for act in actions.iter() {
        let Area(ref p1, ref p2) = act.area;
        for row in lights[p1.x as usize .. (p2.x+1) as usize].iter_mut() {
            for pos in row[p1.y as usize .. (p2.y+1) as usize].iter_mut() {
                match act.change {
                    LightChangeTo::On => *pos = true,
                    LightChangeTo::Off => *pos = false,
                    LightChangeTo::Toggle => *pos = !*pos
                }
            }
        }
    }
    let count : usize = lights.iter().fold(0, |sum, x| sum + x.iter().fold(0, |sum, x| sum + *x as usize ));
    println!("{}", count);
    let mut var_lights : [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    for act in actions.iter() {
        let Area(ref p1, ref p2) = act.area;
        for row in var_lights[p1.x as usize .. (p2.x+1) as usize].iter_mut() {
            for pos in row[p1.y as usize .. (p2.y+1) as usize].iter_mut() {
                match act.change {
                    LightChangeTo::On => *pos += 1,
                    LightChangeTo::Off => {if *pos > 0 {*pos -= 1} },
                    LightChangeTo::Toggle => *pos += 2
                }
            }
        }
    }
    let count2 : usize = var_lights.iter().fold(0, |sum, x| sum + x.iter().fold(0, |sum, x| sum + *x));
    println!("{}", count2);
}
