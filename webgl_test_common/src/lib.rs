extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MapData {
    radius:            usize,
    hexes:             Vec<Vec<Hex>>,
    pub light_sources: Vec<LightSource>,
    pub skybox:        SkyboxCompressed,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Hex {
    pub height: f32,
    pub color:  RgbByteColor,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub struct RgbByteColor(pub [u8; 3]);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum LightSource {
    Directional([f32; 3]),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct SkyboxCompressed {
    pub images: [PngData; 6],
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PngData {
    pub data: Vec<u8>,
}

impl MapData {
    #[inline]
    pub fn new(
        radius: usize,
        hexes: Vec<Vec<Hex>>,
        light_sources: Vec<LightSource>,
        skybox: SkyboxCompressed,
    ) -> Self {
        Self {
            radius,
            hexes,
            light_sources,
            skybox,
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
}

impl Default for MapData {
    #[inline]
    fn default() -> Self {
        Self {
            radius:        0,
            hexes:         Vec::new(),
            light_sources: Vec::new(),
            skybox:        SkyboxCompressed::default(),
        }
    }
}

impl Hex {
    pub fn new(height: f32, color: RgbByteColor) -> Self {
        Self { height, color }
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
}

impl SkyboxCompressed {
    #[inline]
    pub fn new(images: [PngData; 6]) -> Self {
        Self { images }
    }
}

impl Default for SkyboxCompressed {
    #[inline]
    fn default() -> Self {
        Self {
            images: [
                PngData::default(),
                PngData::default(),
                PngData::default(),
                PngData::default(),
                PngData::default(),
                PngData::default(),
            ],
        }
    }
}

impl PngData {
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl Default for PngData {
    #[inline]
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
