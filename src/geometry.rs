use na;

pub const SQRT_3_ON_2: f32 =
    0.866_025_403_784_438_596_588_302_061_718_422_919_511_795_043_945_312;

pub const HEXAGON: [[f32; 2]; 6] = [
    [1.0, 0.0],
    [0.5, SQRT_3_ON_2],
    [-0.5, SQRT_3_ON_2],
    [-1.0, 0.0],
    [-0.5, -SQRT_3_ON_2],
    [0.5, -SQRT_3_ON_2],
];

pub const HEXAGON_TRI_FAN: &[f32] = &[
    0.0,
    0.0,
    //
    1.0,
    0.0,
    //
    0.5,
    SQRT_3_ON_2,
    //
    -0.5,
    SQRT_3_ON_2,
    //
    -1.0,
    0.0,
    //
    -0.5,
    -SQRT_3_ON_2,
    //
    0.5,
    -SQRT_3_ON_2,
    //
    1.0,
    0.0,
];

pub const HEXAGONAL_PRISM_STRIP: &[f32] = &[
    1.0,
    0.0,
    0.0,
    //
    1.0,
    0.0,
    12.0,
    //
    0.5,
    SQRT_3_ON_2,
    0.0,
    //
    0.5,
    SQRT_3_ON_2,
    12.0,
    //
    -0.5,
    SQRT_3_ON_2,
    0.0,
    //
    -0.5,
    SQRT_3_ON_2,
    12.0,
    //
    -1.0,
    0.0,
    0.0,
    //
    -1.0,
    0.0,
    12.0,
    //
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    //
    -0.5,
    -SQRT_3_ON_2,
    12.0,
    //
    0.5,
    -SQRT_3_ON_2,
    0.0,
    //
    0.5,
    -SQRT_3_ON_2,
    12.0,
    //
    1.0,
    0.0,
    0.0,
    //
    1.0,
    0.0,
    12.0,
];

/// Note: This matrix flips the y-axis so 0 is at the top.
pub fn new_projection(
    width: f32,
    height: f32,
    depth: f32,
) -> na::Matrix4<f32> {
    return na::Matrix4::new(
        2.0 / width,
        0.0,
        0.0,
        -1.0,
        //
        0.0,
        -2.0 / height,
        0.0,
        1.0,
        //
        0.0,
        0.0,
        2.0 / depth,
        0.0,
        //
        0.0,
        0.0,
        0.0,
        1.0,
    );
}
