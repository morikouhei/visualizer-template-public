#![allow(non_snake_case, unused_macros)]

use itertools::{max, Itertools};
use proconio::input;
use rand::prelude::*;
// use std::fmt::format;
use std::{collections::BTreeSet, ops::RangeBounds};
// use std::ops::RangeBounds;
use svg::node::element::{Rectangle, Style,Line,Text,Circle, Title,Group};

use svg::node::Text as TextContent;
// use svg::Document;

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

/// 0 <= val <= 1
pub fn color(mut val: f64) -> String {
    val.setmin(1.0);
    val.setmax(0.0);
    let (r, g, b) = if val < 0.5 {
        let x = val * 2.0;
        (
            30. * (1.0 - x) + 144. * x,
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
        )
    } else {
        let x = val * 2.0 - 1.0;
        (
            144. * (1.0 - x) + 255. * x,
            255. * (1.0 - x) + 30. * x,
            30. * (1.0 - x) + 70. * x,
        )
    };
    format!("#{:02x}{:02x}{:02x}", r.round() as i32, g.round() as i32, b.round() as i32)
}

pub fn rect(x: usize, y: usize, w: usize, h: usize, fill: &str) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", w)
        .set("height", h)
        .set("fill", fill)
        // .set("stroke","white")
        .set("stroke-width",1)
}
pub fn txt(x: usize, y: usize, text: &str) -> Text {
    Text::new()
        .add(TextContent::new(text))
        .set("x", x)
        .set("y", y)
        .set("fill", "black")
        .set("font-size", 20)
        .set("dominant-baseline", "central") // 上下中央揃え
        .set("text-anchor", "middle") // 左右中央揃え
}

pub fn circle(x: usize, y: usize, r: usize, stroke: &str) -> Circle {
    Circle::new()
        .set("cx", x)
        .set("cy", y)
        .set("r", r)
        .set("stroke", stroke)
        .set("fill","transparent")
        .set("stroke-width",3)
}
pub fn line(x1: usize, y1: usize, x2: usize, y2: usize, color: &str) -> Line {
    Line::new()
        .set("x1", x1)
        .set("y1", y1)
        .set("x2", x2)
        .set("y2", y2)
        .set("stroke", color)
        .set("stroke-width", 3)
        .set("stroke-linecap", "round")
}

pub fn group(title: String) -> Group {
    Group::new().add(Title::new().add(TextContent::new(title)))
}

pub fn read<T: Copy + PartialOrd + std::fmt::Display + std::str::FromStr, R: RangeBounds<T>>(
    token: Option<&str>,
    range: R,
) -> Result<T, String> {
    if let Some(v) = token {
        if let Ok(v) = v.parse::<T>() {
            if !range.contains(&v) {
                Err(format!("Out of range: {}", v))
            } else {
                Ok(v)
            }
        } else {
            Err(format!("Parse error: {}", v))
        }
    } else {
        Err("Unexpected EOF".to_owned())
    }
}

const DIRS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

pub fn gen(seed: u64) -> Input {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let n = 200;
    let m = 10;
    let mut is = (1..=n).collect_vec();
    is.shuffle(&mut rng);
    let mut bs = vec![vec![]; m];
    for i in 0..n {
        bs[i % m].push(is[i]);
    }
    Input { n, m, bs }
}


// Todo1 Input 関連の定義

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub bs: Vec<Vec<usize>>,
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.m)?;
        for i in 0..self.m {
            writeln!(f, "{}", self.bs[i].iter().join(" "))?;
        }
        Ok(())
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        n: usize, m: usize,
        bs: [[usize; n / m]; m]
    }
    Input { n, m, bs }
}

// Todo2 Output 関連の定義
pub struct Output {
    pub out: Vec<(usize, usize)>,
}

pub fn parse_output(input: &Input, f: &str) -> Result<Output, String> {
    let mut out = vec![];
    let mut tokens = f.split_whitespace().peekable();
    while tokens.peek().is_some() {
        out.push((read(tokens.next(), 1..=input.n)?, read(tokens.next(), 0..=input.m)?));
    }
    if out.len() > 5000 {
        return Err("Too many output".to_owned());
    }
    Ok(Output { out })
}

pub fn compute_score(input: &Input, out: &Output) -> (i64, String) {
    let (mut score, err, _) = compute_score_details(input, &out.out,out.out.len());
    if err.len() > 0 {
        score = 0;
    }
    (score, err)
}

pub fn compute_score_details(input: &Input,
    out: &[(usize,usize)],turn: usize
) -> (i64, String, Vec<Vec<usize>>) {
    
    let mut bs = input.bs.clone();
    let mut cost = 0;
    let mut t = 0;
    let mut trn = 0;
    for &(v, mut to) in out {
        if trn == turn {
            break;
        }
        trn += 1;
        let (i, j) = 'ij: {
            for i in 0..bs.len() {
                for j in 0..bs[i].len() {
                    if bs[i][j] == v {
                        break 'ij (i, j);
                    }
                }
            }
            return (0, format!("Box {v} has already been taken out."), bs);
        };
        if to == 0 {
            if j + 1 != bs[i].len() {
                return (0, format!("Box {v} is not at the top of the stack."), bs);
            } else if v != t + 1 {
                return (
                    0,
                    format!("Before carrying out box {v}, all boxes less than {v} must be carried out."),
                    bs,
                );
            }
            bs[i].pop();
            t += 1;
        } else {
            cost += bs[i].len() - j + 1;
            to -= 1;
            if i != to {
                for k in j..bs[i].len() {
                    let b = bs[i][k];
                    bs[to].push(b);
                }
                bs[i].truncate(j);
            }
        }
    }
    let score = (10000 - cost as i64).max(1);
    let err = if t < input.n {
        format!("Not finished ({} / {})", t, input.n)
    } else {
        String::new()
    };
    (score, err, bs)

}

// Todo3 vis,score 関連の定義
pub fn vis(input: &Input, output: &Output, turn: usize) -> (i64, String, String) {
    let (score, err,bs) =
    compute_score_details(&input, &output.out,turn);
    let W = 800;
    let H = 800;

    let max_h = (2 * input.n / input.m).max(bs.iter().map(|b| b.len()).max().unwrap());
    let w = W / input.m;
    let h = H / max_h;

    let mut doc = svg::Document::new()
        .set("id", "vis")
        .set("viewBox", (-5, -5, W + 10, H + 10))
        .set("width", W + 10)
        .set("height", H + 10)
        .set("style", "background-color:white");

    doc = doc.add(Style::new(format!(
        "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
        16 * 2 * input.n / input.m / max_h
    )));

    // (score, err, doc.to_string())
    
    for i in 0..input.m {
        for j in 0..bs[i].len() {
            doc = doc.add(
                rect(
                    i * w + 1,
                    (max_h - j - 1) * h,
                    w - 2,
                    h,
                    &color(bs[i][j] as f64 / input.n as f64)
                )
            );
            doc = doc.add(
                txt(
                    i * w + w / 2, 
                    (max_h - j - 1) * h + h/2,
                    &bs[i][j].to_string())
            );
        }
    }

    (score, err, doc.to_string())

    // let D = 600 / input.N;
    // let W = D * input.N;
    // let H = D * input.N;
    // let mut doc = svg::Document::new()
    //     .set("id", "vis")
    //     .set("viewBox", (-5, -5, W + 10, H + 10))
    //     .set("width", W + 10)
    //     .set("height", H + 10)
    //     .set("style", "background-color:white");

    // doc = doc.add(Style::new(format!(
    //     "text {{text-anchor: middle;dominant-baseline: central; font-size: {}}}",
    //     6
    // )));
    // for x in 0..input.N {
    //     for y in 0..input.N {
    //         let c = 1.0 - (b[x][y] as f64 / MOD as f64).powi(3);
    //         doc = doc.add(
    //             rect(
    //                 y * D,
    //                 x * D,
    //                 D,
    //                 D,
    //                 &color(c),
    //             )
    //         );
    //         doc = doc.add(
    //             txt(
    //                 y * D + D / 2, 
    //                 x * D + D / 2,
    //                 &b[x][y].to_string()
    //             )
    //             .set("font-size",D/6)
    //         );
    //     }
    // }
    // if turn > 0 {
    //     let (_,x,y) = &output.out[turn-1];
    //     doc = doc.add(
    //         rect(
    //             y * D,
    //             x * D,
    //             3 * D,
    //             3 * D,
    //             "none",
    //         )
    //         .set("stroke","black")
    //         .set("stroke-width",3)
    //     );
        
    // }

    
    // for x in 0..input.N {
    //     for y in 0..input.N-1 {
    //         if input.vs[x][y] == '1'{
    //             doc = doc.add(
    //                 line(
    //                     (y+1)*D,
    //                     x*D,
    //                     (y+1)*D,
    //                     (x+1)*D,
    //                     "black"
    //                 )
    //             )
    //         }
    //     }
    // }
    // for x in 0..input.N-1 {
    //     for y in 0..input.N {
    //         if input.hs[x][y] == '1'{
    //             doc = doc.add(
    //                 line(
    //                     y*D,
    //                     (x+1)*D,
    //                     (y+1)*D,
    //                     (x+1)*D,
    //                     "black"
    //                 )
    //             )
    //         }
    //     }
    // }
    // doc = doc.add(
    //     circle(
    //         p1.1 * D + D/2,
    //         p1.0 * D + D/2,
    //         D/2,
    //         "red"
    //     )
    // );
    // doc = doc.add(
    //     circle(
    //         p2.1 * D + D/2,
    //         p2.0 * D + D/2,
    //         D/2,
    //         "blue"
    //     )
    // );


    // (score as i64, "".to_string(), doc.to_string())
}
