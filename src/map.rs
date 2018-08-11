use bincode;
use geometry;
use std::{iter::Iterator, ops::Index, sync::Mutex};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Map {
    radius:        usize,
    hexes:         Vec<Vec<(Hex, (f32, f32))>>,
    light_sources: Vec<LightSource>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct MapData {
    radius:        usize,
    hexes:         Vec<Vec<Hex>>,
    light_sources: Vec<LightSource>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Hex {
    pub height: f32,
    pub color:  RgbByteColor,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct RgbByteColor([u8; 3]);

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct RgbColor([f32; 3]);

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub enum LightSource {
    Directional([f32; 3]),
}

pub struct MapIter<'a> {
    data: &'a Map,
    row:  usize,
    col:  usize,
}

pub struct MapDataIter<'a> {
    data: &'a MapData,
    row:  usize,
    col:  usize,
}

pub struct MapDataAxialIter<'a> {
    data: &'a MapData,
    row:  usize,
    col:  usize,
}

pub struct MapDataCartesianIter<'a> {
    data: &'a MapData,
    row:  usize,
    col:  usize,
}

lazy_static! {
    pub static ref MAP: Mutex<Map> = Mutex::new(Map::new());
}

impl MapData {
    #[inline]
    pub fn new() -> Self {
        Self {
            radius:        0,
            hexes:         Vec::new(),
            light_sources: Vec::new(),
        }
    }

    #[inline]
    pub fn from_raw_data(data: &[u8]) -> Option<Self> {
        bincode::deserialize(data).ok()
    }

    #[inline]
    pub fn get_radius(&self) -> usize {
        self.radius
    }

    #[inline]
    pub fn get_hexes(&self) -> &Vec<Vec<Hex>> {
        &self.hexes
    }

    #[inline]
    pub fn iter(&self) -> MapDataIter {
        MapDataIter {
            data: &self,
            row:  0,
            col:  0,
        }
    }

    #[inline]
    pub fn axial_iter(&self) -> MapDataAxialIter {
        MapDataAxialIter {
            data: &self,
            row:  0,
            col:  0,
        }
    }

    #[inline]
    pub fn cartesian_iter(&self) -> MapDataCartesianIter {
        MapDataCartesianIter {
            data: &self,
            row:  0,
            col:  0,
        }
    }
}

impl Map {
    #[inline]
    pub fn new() -> Self {
        Self {
            radius:        0,
            hexes:         Vec::new(),
            light_sources: Vec::new(),
        }
    }

    pub fn from_map_data(md: MapData) -> Self {
        let mut hexes = Vec::with_capacity(md.hexes.len());
        for (row_n, row) in md.hexes.into_iter().enumerate() {
            let mut new_row = Vec::with_capacity(row.len());

            for (col_n, col) in row.into_iter().enumerate() {
                let (q, r) = (col_n + md.radius.saturating_sub(row_n), row_n);
                new_row.push((
                    col,
                    geometry::axial_to_cartesian(q as f32, r as f32),
                ));
            }

            hexes.push(new_row);
        }

        Self {
            radius: md.radius,
            hexes,
            light_sources: md.light_sources,
        }
    }

    #[inline]
    pub fn get_radius(&self) -> usize {
        self.radius
    }

    #[inline]
    pub fn get_hexes(&self) -> &Vec<Vec<(Hex, (f32, f32))>> {
        &self.hexes
    }

    #[inline]
    pub fn iter(&self) -> MapIter {
        MapIter {
            data: &self,
            row:  0,
            col:  0,
        }
    }
}

impl RgbByteColor {
    #[inline]
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn rgb(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn rgba(&self) -> [u8; 4] {
        [self.0[0], self.0[1], self.0[2], 0xFF]
    }

    #[inline]
    pub fn as_floating(&self) -> RgbColor {
        RgbColor([
            f32::from(self.0[0]) / 255.0,
            f32::from(self.0[1]) / 255.0,
            f32::from(self.0[2]) / 255.0,
        ])
    }
}

impl RgbColor {
    #[inline]
    pub fn r(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    pub fn rgb(&self) -> &[f32] {
        &self.0
    }

    #[inline]
    pub fn rgba(&self) -> [f32; 4] {
        [self.0[0], self.0[1], self.0[2], 1.0]
    }

    #[inline]
    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.0[0], self.0[1], self.0[2])
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = &'a (Hex, (f32, f32));

    fn next(&mut self) -> Option<Self::Item> {
        let hexes = self.data.get_hexes();
        let h = hexes[self.row].get(self.col);
        if h.is_some() {
            self.col += 1;

            h
        } else {
            self.row += 1;
            self.col = 1;

            hexes.get(self.row).map(|r| r.index(0))
        }
    }
}

impl<'a> MapDataIter<'a> {
    #[inline]
    pub fn with_axial(self) -> MapDataAxialIter<'a> {
        MapDataAxialIter {
            data: self.data,
            row:  self.row,
            col:  self.col,
        }
    }

    #[inline]
    pub fn with_cartesian(self) -> MapDataCartesianIter<'a> {
        MapDataCartesianIter {
            data: self.data,
            row:  self.row,
            col:  self.col,
        }
    }
}

impl<'a> Iterator for MapDataIter<'a> {
    type Item = &'a Hex;

    fn next(&mut self) -> Option<Self::Item> {
        let hexes = self.data.get_hexes();
        let h = hexes[self.row].get(self.col);
        if h.is_some() {
            self.col += 1;

            h
        } else {
            self.row += 1;
            self.col = 1;

            hexes.get(self.row).map(|r| r.index(0))
        }
    }
}

impl<'a> MapDataAxialIter<'a> {
    #[inline]
    pub fn just_hexes(self) -> MapDataIter<'a> {
        MapDataIter {
            data: self.data,
            row:  self.row,
            col:  self.col,
        }
    }

    #[inline]
    pub fn as_cartesian(&self) -> MapDataCartesianIter<'a> {
        MapDataCartesianIter {
            data: self.data,
            row:  self.row,
            col:  self.col,
        }
    }
}

impl<'a> Iterator for MapDataAxialIter<'a> {
    type Item = (&'a Hex, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let hexes = self.data.get_hexes();
        if let Some(h) = hexes[self.row].get(self.col) {
            let ret_col = self.col;
            self.col += 1;

            Some((
                h,
                ret_col + self.data.radius.saturating_sub(self.row),
                self.row,
            ))
        } else {
            self.row += 1;
            self.col = 1;

            hexes.get(self.row).map(|r| {
                (
                    r.index(0),
                    self.data.radius.saturating_sub(self.row),
                    self.row,
                )
            })
        }
    }
}

impl<'a> MapDataCartesianIter<'a> {
    #[inline]
    pub fn just_hexes(self) -> MapDataIter<'a> {
        MapDataIter {
            data: self.data,
            row:  self.row,
            col:  self.col,
        }
    }

    #[inline]
    pub fn as_axial(&self) -> MapDataAxialIter<'a> {
        MapDataAxialIter {
            data: self.data,
            row:  self.row,
            col:  self.col,
        }
    }
}

impl<'a> Iterator for MapDataCartesianIter<'a> {
    type Item = (&'a Hex, f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let hexes = self.data.get_hexes();
        if let Some(h) = hexes[self.row].get(self.col) {
            let ret_col = self.col;
            self.col += 1;

            let (q, r) = (
                ret_col + self.data.radius.saturating_sub(self.row),
                self.row,
            );
            let (x, y) = geometry::axial_to_cartesian(q as f32, r as f32);

            Some((h, x, y))
        } else {
            self.row += 1;
            self.col = 1;

            let (q, r) = (self.data.radius.saturating_sub(self.row), self.row);
            let (x, y) = geometry::axial_to_cartesian(q as f32, r as f32);

            hexes.get(self.row).map(|r| (r.index(0), x, y))
        }
    }
}
