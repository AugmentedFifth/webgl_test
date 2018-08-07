use random;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MapData {
    radius:        usize,
    hexes:         Vec<Vec<Hex>>,
    light_sources: Vec<LightSource>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Hex {
    pub height: f32,
    pub color:  RgbByteColor,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct RgbByteColor([u8; 3]);

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum LightSource {
    Directional([f32; 3]),
}

struct CubeRing {
    cube:   Cube,
    radius: usize,
    i:      usize,
    j:      usize,
}

type Cube = (isize, isize, isize);
type Axial = (isize, isize);

const STAY_PROB: f32 = 0.75;
const STEP_SIZE: f32 = 0.5;

impl Hex {
    pub fn new(height: f32, color: RgbByteColor) -> Self {
        Self { height, color }
    }
}

impl RgbByteColor {
    pub fn new_random() -> Self {
        RgbByteColor(random::gen())
    }
}

#[inline]
fn cube_add(c0: Cube, c1: Cube) -> Cube {
    (c0.0 + c1.0, c0.1 + c1.1, c0.2 + c1.2)
}

#[inline]
fn cube_scale(c: Cube, a: isize) -> Cube {
    (c.0 * a, c.1 * a, c.2 * a)
}

#[inline]
fn cube_direction(direction: usize) -> Cube {
    let cube_directions = [
        (1, -1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (-1, 1, 0),
        (-1, 0, 1),
        (0, -1, 1),
    ];

    cube_directions[direction]
}

#[inline]
fn cube_neighbor(c: Cube, direction: usize) -> Cube {
    cube_add(c, cube_direction(direction))
}

/// Doesn't work for `radius == 0`.
#[inline]
fn cube_ring(center: Cube, radius: usize) -> CubeRing {
    CubeRing {
        cube: cube_add(center, cube_scale(cube_direction(4), radius as isize)),
        radius,
        i: 0,
        j: 0,
    }
}

impl Iterator for CubeRing {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 6 {
            if self.j < self.radius {
                self.j += 1;
                let ret = self.cube;
                self.cube = cube_neighbor(self.cube, self.i);

                Some(ret)
            } else {
                self.j = 1;
                self.i += 1;
                if self.i < 6 {
                    let ret = self.cube;
                    self.cube = cube_neighbor(self.cube, self.i);

                    Some(ret)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

#[inline]
fn cube_to_axial(c: Cube) -> Axial {
    (c.0, c.2)
}

#[inline]
fn axial_to_cube(axial: Axial) -> Cube {
    (axial.0, -(axial.0 + axial.1), axial.1)
}

#[inline]
fn axial_to_indices(axial: Axial, radius: usize) -> (usize, usize) {
    (
        axial.1 as usize,
        axial.0 as usize - radius.saturating_sub(axial.1 as usize),
    )
}

pub fn generate_map(radius: usize) -> MapData {
    let (a, b) = (radius + 1, 2 * radius + 1);
    let mut hexes = Vec::with_capacity(b);
    let mut hex_parents = Vec::with_capacity(b);
    for n in (a..b).chain((a..=b).rev()) {
        let mut v = Vec::with_capacity(n);
        unsafe {
            v.set_len(n);
        }
        hexes.push(v);

        let mut v = Vec::with_capacity(n);
        unsafe {
            v.set_len(n);
        }
        hex_parents.push(v);
    }

    hexes[radius][radius] = Hex::new(0.0, RgbByteColor::new_random());
    hex_parents[radius][radius] = (0.0, 0);

    let rad = radius as isize;
    let distance_from_center = |(c0, c1, c2): Cube| {
        (c0 - rad)
            .abs()
            .max((c1 + 2 * rad).abs())
            .max((c2 - rad).abs())
    };
    for i in 1..=radius {
        for c in cube_ring(axial_to_cube((rad, rad)), i) {
            let (mut parent0, mut parent1) = (None, None);
            for possible_parent in cube_ring(c, 1) {
                let possible_distance = distance_from_center(possible_parent);
                if let Some((_, p0_dist)) = parent0 {
                    if possible_distance < p0_dist {
                        parent0 = Some((possible_parent, possible_distance));
                        parent1 = None;
                    } else if possible_distance == p0_dist {
                        parent1 = Some(possible_parent);
                    }
                } else {
                    parent0 = Some((possible_parent, possible_distance));
                }
            }

            let (parent_height, parent_dir) = {
                let hex_index = |(q, _, r): Cube| {
                    let (i, j) = axial_to_indices((q, r), radius);
                    hex_parents[i][j]
                };

                if let Some(p1) = parent1 {
                    if random::gen() {
                        hex_index(parent0.unwrap().0)
                    } else {
                        hex_index(p1)
                    }
                } else {
                    hex_index(parent0.unwrap().0)
                }
            };

            let our_dir = if STAY_PROB > random::gen() {
                parent_dir
            } else {
                let (dir0, dir1) = match parent_dir {
                    0 => (-1, 1),
                    1 => (-1, 0),
                    _ => (0, 1),
                };

                if random::gen() {
                    dir0
                } else {
                    dir1
                }
            };
            let our_height = parent_height + our_dir as f32 * STEP_SIZE;

            let (i, j) = axial_to_indices(cube_to_axial(c), radius);
            hexes[i][j] = Hex::new(our_height, RgbByteColor::new_random());
            hex_parents[i][j] = (our_height, our_dir);
        }
    }

    MapData {
        radius,
        hexes,
        light_sources: Vec::new(),
    }
}
