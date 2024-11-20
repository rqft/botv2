use std::{
    borrow::Cow,
    collections::HashMap,
    io::{BufWriter, Cursor},
    str::FromStr,
    time::Duration,
};

use image::{
    codecs::gif::{GifEncoder, Repeat},
    Delay, Frame, GenericImageView, Rgba,
};
use lazy_static::lazy_static;
use regex::Regex;

pub const BW: usize = 20;
pub const BH: usize = 20;
pub const HL: usize = 4;
pub const SHADING: u32 = 4;
pub const PADDING: u32 = 5;
pub const SHADOW: Rgba<u8> = Rgba([0x26, 0x26, 0x2a, 0xff]);
// pub const BG: Rgba<u8> = Rgba([0x36, 0x39, 0x41, 0xff]);
pub fn render_grid(g: &str, lp: bool, spec: bool, lcs: bool, delay: Duration) -> Vec<u8> {
    let buf = Cursor::new(vec![]);
    let mut writer = BufWriter::new(buf);
    let mut ge = GifEncoder::new(&mut writer);
    ge.set_repeat(if lp {
        Repeat::Infinite
    } else {
        Repeat::Finite(0)
    })
    .unwrap();
    let grids = g.split(";");
    for g in grids {
        let grid = g
            .split("|")
            .map(|v| v.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        // let longest = grid.iter().map(|v| v.len()).max().unwrap();
        // println!("{}, {}", grid[0].len(), grid.len());
        let ng = preprocess_grid(grid).unwrap();
        let mut img = image::RgbaImage::new(
            ng[0].len() as u32 * BW as u32 + (2 * PADDING) + SHADING as u32,
            ng.len() as u32 * BH as u32 + (2 * PADDING) + HL as u32 + SHADING as u32,
        );

        for (i, r) in ng.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                let has_air = i
                    .checked_sub(1)
                    .and_then(|v| ng.get(v))
                    .and_then(|v| v.get(j))
                    .copied()
                    .unwrap_or(Piece::E)
                    == Piece::E;
                let is_line_clear = !r.contains(&Piece::E);
                // dbg!(is_line_clear, r);
                let p = Piece::from_str(&c.to_string()).unwrap();
                // main
                for pi in i * BW..(i + 1) * BW {
                    for pj in j * BW..(j + 1) * BW {
                        let pix = Rgba::from(p.color().to_be_bytes());
                        img.put_pixel(
                            pj as u32 + PADDING,
                            pi as u32 + PADDING,
                            if lcs && is_line_clear {
                                apply_filters(pix)
                            } else {
                                pix
                            },
                        );
                    }
                }

                for pi in i * BW..i * BW + HL {
                    for pj in j * BW..(j + 1) * BW {
                        let pix = Rgba::from(p.color_bright().to_be_bytes());
                        if has_air {
                            img.put_pixel(
                                pj as u32 + PADDING,
                                pi as u32 + PADDING - HL as u32,
                                pix,
                            );
                        }
                    }
                }

                // let cpy = img.clone();
                // for (x, y, p) in cpy.enumerate_pixels() {
                //     if let Some(s) = cpy.get_pixel_checked(x + SHADING, y + SHADING) {
                //         if s.0[3] == 0 && p.0[3] != 0 && *p != SHADOW {
                //             img.put_pixel(x + SHADING, y + SHADING, SHADOW);
                //         }
                //     }
                // }
            }
        }

        ge.encode_frame(Frame::from_parts(
            img,
            0,
            0,
            Delay::from_saturating_duration(delay),
        ))
        .unwrap();
    }

    drop(ge);
    writer.into_inner().unwrap().into_inner()
}

fn preprocess_grid(mut grid: Vec<Vec<char>>) -> anyhow::Result<Grid> {
    let mut ng = vec![];
    for i in grid {
        let mut nr = vec![];
        for (j, c) in i.iter().enumerate() {
            if let Some(t) = c.to_digit(10) {
                if t == 0 {
                    continue;
                }
                for v in 0..t - 1 {
                    nr.push(
                        *i.get(j - 1)
                            .ok_or(anyhow::anyhow!("missing piece for quantifier"))?,
                    );
                }
            } else {
                nr.push(*c);
            }
        }

        ng.push(nr);
    }

    for i in ng.iter_mut() {
        while i.ends_with(&['e']) {
            i.pop();
        }
    }

    let longest = ng.iter().map(Vec::len).max().unwrap_or(0);
    for i in ng.iter_mut() {
        while i.len() < longest {
            i.push('e');
        }
    }

    Ok(ng
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| Piece::from_str(&y.to_string()).unwrap())
                .collect()
        })
        .collect())
}

fn brighten(mut pixel: Rgba<u8>, scale: f64) -> Rgba<u8> {
    for i in 0..3 {
        // Skip the alpha channel
        let value = (pixel[i] as f64 * scale).clamp(0.0, 255.0);
        pixel[i] = value as u8;
    }

    pixel
}

pub fn apply_filters(color: Rgba<u8>) -> Rgba<u8> {
    let Rgba([r, g, b, a]) = color;

    // Apply brightness (multiply by 1.2)
    let r = (r as f32 * 1.3).min(255.0).round() as u8;
    let g = (g as f32 * 1.3).min(255.0).round() as u8;
    let b = (b as f32 * 1.3).min(255.0).round() as u8;

    // Convert RGB to HSL to apply saturation
    let (h, s, l) = rgb_to_hsl(r, g, b);

    // Apply saturation (multiply by 0.8)
    let new_s = (s * 0.8).min(1.0);

    // Convert back to RGB
    let (r, g, b) = hsl_to_rgb(h, new_s, l);

    Rgba([r, g, b, a])
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return (0.0, 0.0, l); // achromatic
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = match max {
        _ if max == r => (g - b) / d + if g < b { 6.0 } else { 0.0 },
        _ if max == g => (b - r) / d + 2.0,
        _ if max == b => (r - g) / d + 4.0,
        _ => 0.0, // just in case, should never happen
    } / 6.0;

    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let (r, g, b);

    if s == 0.0 {
        r = l;
        g = l;
        b = l; // achromatic
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    (
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    )
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let mut t = t;
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Piece {
    I,
    J,
    L,
    O,
    Z,
    S,
    T,
    G,
    E,
    D,
}

pub type Grid = Vec<Vec<Piece>>;

impl Piece {
    pub fn color(&self) -> u32 {
        match self {
            Self::I => 0x42afe1ff,
            Self::L => 0xf38927ff,
            Self::J => 0x1165b5ff,
            Self::O => 0xf6d03cff,
            Self::Z => 0xeb4f65ff,
            Self::S => 0x51b84dff,
            Self::T => 0x9739a2ff,
            Self::G => 0x868686ff,
            // Self::E => 0x36394100,
            Self::E => 0x00000000,
            Self::D => 0x434343ff,
        }
    }

    pub fn color_bright(&self) -> u32 {
        match self {
            Self::I => 0x6ceaffff,
            Self::L => 0xffba59ff,
            Self::J => 0x339bffff,
            Self::O => 0xffff7fff,
            Self::Z => 0xff7f79ff,
            Self::S => 0x84f880ff,
            Self::T => 0xd958e9ff,
            Self::G => 0xddddddff,
            // Self::E => 0x36394100,
            Self::E => 0x00000000,
            Self::D => 0x777777ff
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::I => "I",
                Self::J => "J",
                Self::L => "L",
                Self::O => "O",
                Self::Z => "Z",
                Self::S => "S",
                Self::T => "T",
                Self::G => "G",
                Self::E => "E",
                Self::D => "D",
            }
        )
    }
}

impl FromStr for Piece {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_uppercase() {
            "I" => Ok(Self::I),
            "J" => Ok(Self::J),
            "L" => Ok(Self::L),
            "O" => Ok(Self::O),
            "Z" => Ok(Self::Z),
            "S" => Ok(Self::S),
            "T" => Ok(Self::T),
            "G" => Ok(Self::G),
            "E" => Ok(Self::E),
            "D" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

pub fn data() -> String {
    std::fs::read_to_string("t_data.txt").unwrap()
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub id: String,
    pub grid: Grid,
    pub continuations: Vec<(Piece, Grid)>,
}

pub type Resp = HashMap<u8, Vec<Pattern>>;

lazy_static! {
    static ref RES_R: Regex = regex::Regex::new(r"(\d+):(\w+)=(.+?)#(.*)").unwrap();
}
// Equivalent function for `resp`.
pub fn resp() -> HashMap<u8, Vec<Pattern>> {
    let mut resp: HashMap<u8, Vec<Pattern>> = HashMap::new();
    // println!("1");

    for line in data().lines() {
        if let Some(captures) = RES_R.captures(line) {
            let r = captures.get(1).unwrap().as_str().parse().unwrap();
            let id = captures.get(2).unwrap().as_str().to_string();
            let grid = captures.get(3).unwrap().as_str();
            let conts = captures.get(4).unwrap().as_str();

            let g2: Grid = grid
                .split('|')
                .map(|x| {
                    x.chars()
                        .map(|y| Piece::from_str(&y.to_string()).unwrap())
                        .collect()
                })
                .collect();

            let c2: Vec<(Piece, Grid)> = conts
                .split(';')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    let parts: Vec<&str> = x.split(',').collect();
                    let piece = Piece::from_str(parts[0]).unwrap();
                    let grid: Grid = parts[1]
                        .split('|')
                        .map(|y| {
                            y.chars()
                                .map(|z| Piece::from_str(&z.to_string()).unwrap())
                                .collect()
                        })
                        .collect();
                    (piece, grid)
                })
                .collect();

            let pattern = Pattern {
                id,
                grid: g2,
                continuations: c2,
            };
            resp.entry(r).or_insert_with(Vec::new).push(pattern);
        }
    }

    resp
}

pub type Pc = (bool, u8, Vec<Piece>, Grid);

// Equivalent function for `pcs`.
fn pcs() -> Vec<Pc> {
    let mut resp: Vec<Pc> = Vec::new();

    for line in data().lines() {
        if let Some(captures) = regex::Regex::new(r"p(c|n)(\d+):(\w+)=(.+)")
            .unwrap()
            .captures(line)
        {
            let ty = captures.get(1).unwrap().as_str();
            let r: u8 = captures.get(2).unwrap().as_str().parse().unwrap();
            let pieces = captures.get(3).unwrap().as_str();
            let grid = captures.get(4).unwrap().as_str();

            if resp.iter().any(|x| to_grid(&x.3) == grid) {
                continue;
            }

            let pieces_vec: Vec<Piece> = pieces
                .chars()
                .map(|x| Piece::from_str(&x.to_string()).unwrap())
                .collect();
            let grid_vec: Grid = grid
                .split('|')
                .map(|x| {
                    x.chars()
                        .map(|y| Piece::from_str(&y.to_string()).unwrap())
                        .collect()
                })
                .collect();

            resp.push((ty == "c", r, pieces_vec, grid_vec));
        }
    }

    resp
}

pub fn are_grids_equal(mut grid1: Grid, mut grid2: Grid) -> bool {
    // Handle the empty grid cases first.
    if grid1.is_empty() {
        return grid2.is_empty();
    }
    if grid2.is_empty() {
        return grid1.is_empty();
    }

    // Remove rows with only `Piece::E` (empty pieces) from the top of each grid.
    while let Some(first_row) = grid1.first() {
        if first_row.iter().all(|x| *x == Piece::E) {
            grid1.remove(0);
        } else {
            break;
        }
    }

    while let Some(first_row) = grid2.first() {
        if first_row.iter().all(|x| *x == Piece::E) {
            grid2.remove(0);
        } else {
            break;
        }
    }

    // Compare each cell in the two grids.
    if grid1.len() != grid2.len() {
        return false;
    }
    for (row1, row2) in grid1.iter_mut().zip(grid2.iter_mut()) {
        while row1.ends_with(&[Piece::E]) {
            row1.pop();
        }

        while row2.ends_with(&[Piece::E]) {
            row2.pop();
        }

        if row1.len() != row2.len() {
            return false;
        }
        for (cell1, cell2) in row1.iter().zip(row2.iter()) {
            if cell1 != cell2 {
                return false;
            }
        }
    }

    true
}

pub fn after_line_clear<'a>(g: &'a Grid, patterns: &'a [Pattern]) -> Option<&'a Pattern> {
    let v = g.iter().filter(|x| x.contains(&Piece::E));
    let grey: Vec<Vec<Piece>> = v
        .map(|x| {
            x.iter()
                .map(|y| if *y == Piece::E { Piece::E } else { Piece::G })
                .collect()
        })
        .collect();
    patterns
        .iter()
        .find(|x| are_grids_equal(x.grid.clone(), grey.clone()))
}

pub fn parse_grid(t: String) -> Grid {
    preprocess_grid(
        t.split('|')
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect(),
    )
    .unwrap()
}

pub fn to_grid(v: &Grid) -> String {
    v.iter()
        .map(|x| {
            x.iter()
                .map(|y| y.to_string().to_lowercase())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("|")
}

pub mod ren {
    use std::{
        cell::{Cell, RefCell},
        future::Future,
        pin::Pin,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    use async_recursion::async_recursion;
    use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
    use tokio::task;

    use crate::sf::after_line_clear;

    use super::{Grid, Pattern, Piece};

    pub struct State {
        pub patterns: Vec<Pattern>,
        pub board: Pattern,
        pub queue: Vec<Piece>,
        pub hold: Option<Piece>,
    }

    pub type PathItem = (Pattern, Piece, Grid);
    pub type Path = Vec<PathItem>;
    pub type Continuation = (UsedPiece, (Piece, Vec<Vec<Piece>>));

    pub fn pathfind(state: State) -> Path {
        // println!("finding!");
        if state.queue.is_empty() {
            if let Some(h) = state.hold {
                return pathfind(State {
                    queue: vec![h],
                    board: state.board,
                    patterns: state.patterns,
                    hold: None,
                });
            }

            return vec![];
        }

        let cp = state.queue[0];
        let np = state.queue.get(1);
        let hp = state.hold;

        let sbc = state.board.continuations.clone();
        let possible_continuations = vec![
            sbc.clone()
                .into_iter()
                .filter(|x| x.0 == cp)
                .map(|x| (UsedPiece::Current, x))
                .collect::<Vec<_>>(),
            if let Some(h) = hp {
                sbc.clone()
                    .into_iter()
                    .filter(|x| x.0 == h)
                    .map(|x| (UsedPiece::Hold, x))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            },
            if let Some(n) = np
                && hp.is_none()
            {
                sbc.clone()
                    .into_iter()
                    .filter(|x| x.0 == *n)
                    .map(|x| (UsedPiece::Next, x))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            },
        ]
        .concat();

        let mut max = vec![];
        // dbg!(possible_continuations.len());
        let m = Arc::new(Mutex::new(max));
        possible_continuations
            .into_par_iter()
            .for_each(|x| check_continuation(m.clone(), &state, x.clone()));

        let x = m.lock().unwrap().to_vec();
        x
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum UsedPiece {
        Current,
        Next,
        Hold,
    }

    pub fn check_continuation(mut max: Arc<Mutex<Path>>, state: &State, pc: Continuation) {
        let p = state.patterns.clone();
        let q = state.queue.clone();
        let b = state.board.clone();

        let (used, cont) = pc;

        if let Some(board) = after_line_clear(&cont.1, &p) {
            let ns = State {
                patterns: p.clone(),
                board: board.clone(),
                queue: q[if used == UsedPiece::Next { 2.. } else { 1.. }].to_vec(),
                hold: if used == UsedPiece::Current {
                    state.hold
                } else {
                    Some(state.queue[0])
                },
            };

            let sf = pathfind(ns);
            let nw = [vec![(b, cont.0, cont.1)], sf].concat();
            let mut mx = max.lock().unwrap();
            if nw.len() > mx.len()
                || (nw.len() == mx.len()
                    && !nw.is_empty()
                    && !mx.is_empty()
                    && nw.last().unwrap().0.continuations.len()
                        > mx.last().unwrap().0.continuations.len())
            {
                *mx = nw;
            }
        }
    }
}
