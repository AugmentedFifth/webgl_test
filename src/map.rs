use geometry::{self, CubeCoord, CubeRing};
use png;
use std::{iter::Iterator, ops::Index, sync::Mutex};
use webgl;
use webgl_test_common::{
    Hex,
    LightSource,
    MapData,
    RgbByteColor,
    SkyboxCompressed,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Map {
    radius:            usize,
    hexes:             Vec<Vec<(Hex, (f32, f32))>>,
    pub light_sources: Vec<LightSource>,
    pub skybox:        Skybox,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RgbColor([f32; 3]);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Skybox {
    images: [ImgData; 6],
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ImgData {
    data:               Vec<u8>,
    width:              u32,
    height:             u32,
    alpha:              bool,
    eight_bit_channels: bool,
}

pub struct SkyboxImgIter<'a> {
    skybox: &'a Skybox,
    i:      usize,
}

pub struct MapIter<'a> {
    data: &'a Map,
    row:  usize,
    col:  usize,
}

pub struct MapIterRadial<'a> {
    data:   &'a Map,
    center: CubeCoord,
    ring:   CubeRing,
}

lazy_static! {
    pub static ref MAP: Mutex<Map> = Mutex::new(Map::new());
}

impl Map {
    #[inline]
    pub fn new() -> Self {
        Self {
            radius:        0,
            hexes:         Vec::new(),
            light_sources: Vec::new(),
            skybox:        Skybox::default(),
        }
    }

    pub fn from_map_data(md: &MapData) -> Self {
        let mut hexes = Vec::with_capacity(md.get_hexes().len());
        for (row_n, row) in md.get_hexes().into_iter().enumerate() {
            let mut new_row = Vec::with_capacity(row.len());

            for (col_n, col) in row.into_iter().enumerate() {
                let (q, r) =
                    (col_n + md.get_radius().saturating_sub(row_n), row_n);
                new_row.push((
                    col.clone(),
                    geometry::axial_to_cartesian(q as f32, r as f32),
                ));
            }

            hexes.push(new_row);
        }

        Self {
            radius: md.get_radius(),
            hexes,
            light_sources: md.light_sources.clone(),
            skybox: Skybox::from_compressed(&md.skybox),
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
    pub fn index_by_cube(&self, cc: CubeCoord) -> Option<&(Hex, (f32, f32))> {
        let (i, j) = geometry::cube_to_indices(cc, self.radius);
        self.hexes.get(i).and_then(|r| r.get(j))
    }

    #[inline]
    pub fn iter(&self) -> MapIter {
        MapIter {
            data: &self,
            row:  0,
            col:  0,
        }
    }

    /// Returns an iterator over all hexes **except** the `center`, starting
    /// with the hexes closest to `center` and going further and further away.
    #[inline]
    pub fn iter_radial(&self, center: CubeCoord) -> MapIterRadial {
        MapIterRadial {
            data: &self,
            center,
            ring: CubeRing::new(center, 1),
        }
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

    #[inline]
    pub fn from_byte_color(bc: RgbByteColor) -> Self {
        RgbColor([
            f32::from(bc.r()) / 255.0,
            f32::from(bc.g()) / 255.0,
            f32::from(bc.b()) / 255.0,
        ])
    }
}

impl Skybox {
    #[inline]
    pub fn get_images(&self) -> &[ImgData; 6] {
        &self.images
    }

    #[inline]
    pub fn img_iter(&self) -> SkyboxImgIter {
        SkyboxImgIter {
            skybox: &self,
            i:      0,
        }
    }

    pub fn from_compressed(sc: &SkyboxCompressed) -> Self {
        let mut skybox = Skybox::default();

        for (i, p) in sc.images.iter().enumerate() {
            let decoder = png::Decoder::new(p.data.as_slice());
            let (info, mut reader) = decoder.read_info().unwrap();
            let mut buf = Vec::with_capacity(info.buffer_size());
            unsafe {
                buf.set_len(info.buffer_size());
            }
            reader.next_frame(&mut buf).unwrap();

            skybox.images[i] = ImgData::new(
                buf,
                info.width,
                info.height,
                match info.color_type {
                    png::ColorType::RGB => false,
                    png::ColorType::RGBA => true,
                    _ => panic!(),
                },
                match info.bit_depth {
                    png::BitDepth::Eight => true,
                    png::BitDepth::Sixteen => false,
                    _ => panic!(),
                },
            );
        }

        skybox
    }
}

impl Default for Skybox {
    #[inline]
    fn default() -> Self {
        Self {
            images: [
                ImgData::default(),
                ImgData::default(),
                ImgData::default(),
                ImgData::default(),
                ImgData::default(),
                ImgData::default(),
            ],
        }
    }
}

impl<'a> Iterator for SkyboxImgIter<'a> {
    type Item = (&'a ImgData, webgl::TextureBindPoint);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let tbp = match self.i {
            0 => webgl::TextureBindPoint::TextureCubeMapPositiveX,
            1 => webgl::TextureBindPoint::TextureCubeMapNegativeX,
            2 => webgl::TextureBindPoint::TextureCubeMapPositiveY,
            3 => webgl::TextureBindPoint::TextureCubeMapNegativeY,
            4 => webgl::TextureBindPoint::TextureCubeMapPositiveZ,
            5 => webgl::TextureBindPoint::TextureCubeMapNegativeZ,
            _ => return None,
        };

        let ret = (&self.skybox.images[self.i], tbp);
        self.i += 1;
        Some(ret)
    }
}

impl ImgData {
    #[inline]
    pub fn new(
        data: Vec<u8>,
        width: u32,
        height: u32,
        alpha: bool,
        eight_bit_channels: bool,
    ) -> Self {
        Self {
            data,
            width,
            height,
            alpha,
            eight_bit_channels,
        }
    }

    #[inline]
    pub fn has_eight_bit_channels(&self) -> bool {
        self.eight_bit_channels
    }

    #[inline]
    pub fn has_alpha(&self) -> bool {
        self.alpha
    }

    #[inline]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl Default for ImgData {
    #[inline]
    fn default() -> Self {
        Self {
            data:               Vec::new(),
            width:              0,
            height:             0,
            alpha:              false,
            eight_bit_channels: false,
        }
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

impl<'a> Iterator for MapIterRadial<'a> {
    type Item = &'a (Hex, (f32, f32));

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(cc) = self.ring.next() {
            let entry = self.data.index_by_cube(cc);
            if entry.is_some() {
                return entry;
            }
        }

        self.ring = CubeRing::new(self.center, self.ring.get_radius() + 1);

        while let Some(cc) = self.ring.next() {
            let entry = self.data.index_by_cube(cc);
            if entry.is_some() {
                return entry;
            }
        }

        None
    }
}
