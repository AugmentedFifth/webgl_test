pub const SQRT_3_ON_2: f32 =
    0.866_025_403_784_438_596_588_302_061_718_422_919_511_795_043_945_312;

pub const SQRT_3: f32 =
    1.732_050_807_568_877_293_527_446_341_505_872_366_942_805_253_810_380;

pub const HEXAGON: [[f32; 2]; 6] = [
    [1.0, 0.0],
    [0.5, SQRT_3_ON_2],
    [-0.5, SQRT_3_ON_2],
    [-1.0, 0.0],
    [-0.5, -SQRT_3_ON_2],
    [0.5, -SQRT_3_ON_2],
];

pub const HEXAGON_VERTS: &[f32] = &[
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
];

pub const HEXAGON_INDICES: &[u16] = &[
    0, 1, 2, //
    0, 2, 3, //
    0, 3, 4, //
    0, 4, 5, //
    0, 5, 6, //
    0, 6, 1, //
];

pub const HEXAGONAL_PRISM_VERTS: &[f32] = &[
    0.0,
    0.0,
    0.0,
    //
    1.0,
    0.0,
    0.0,
    //
    0.5,
    SQRT_3_ON_2,
    0.0,
    //
    -0.5,
    SQRT_3_ON_2,
    0.0,
    //
    -1.0,
    0.0,
    0.0,
    //
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    //
    0.5,
    -SQRT_3_ON_2,
    0.0,
    //
    1.0,
    0.0,
    -12.0,
    //
    0.5,
    SQRT_3_ON_2,
    -12.0,
    //
    -0.5,
    SQRT_3_ON_2,
    -12.0,
    //
    -1.0,
    0.0,
    -12.0,
    //
    -0.5,
    -SQRT_3_ON_2,
    -12.0,
    //
    0.5,
    -SQRT_3_ON_2,
    -12.0,
];

pub const HEXAGONAL_PRISM_INDICES: &[u16] = &[
    //////////////// Hexagon ////////////////
    0, 1, 2, //
    0, 2, 3, //
    0, 3, 4, //
    0, 4, 5, //
    0, 5, 6, //
    0, 6, 1, //
    //////////////// Prism ////////////////
    1, 7, 2, //
    2, 7, 8, //
    2, 8, 3, //
    3, 8, 9, //
    3, 9, 4, //
    4, 9, 10, //
    4, 10, 5, //
    5, 10, 11, //
    5, 11, 6, //
    6, 11, 12, //
    6, 12, 1, //
    1, 12, 7, //
];

pub const HEXAGONAL_PRISM: &[f32] = &[
    0.0,
    0.0,
    0.0,
    1.0,
    0.0,
    0.0,
    0.5,
    SQRT_3_ON_2,
    0.0,
    0.0,
    0.0,
    0.0,
    0.5,
    SQRT_3_ON_2,
    0.0,
    -0.5,
    SQRT_3_ON_2,
    0.0,
    0.0,
    0.0,
    0.0,
    -0.5,
    SQRT_3_ON_2,
    0.0,
    -1.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    -1.0,
    0.0,
    0.0,
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    0.0,
    0.0,
    0.0,
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    0.5,
    -SQRT_3_ON_2,
    0.0,
    0.0,
    0.0,
    0.0,
    0.5,
    -SQRT_3_ON_2,
    0.0,
    1.0,
    0.0,
    0.0,
    1.0,
    0.0,
    0.0,
    1.0,
    0.0,
    -12.0,
    0.5,
    SQRT_3_ON_2,
    0.0,
    0.5,
    SQRT_3_ON_2,
    0.0,
    1.0,
    0.0,
    -12.0,
    0.5,
    SQRT_3_ON_2,
    -12.0,
    0.5,
    SQRT_3_ON_2,
    0.0,
    0.5,
    SQRT_3_ON_2,
    -12.0,
    -0.5,
    SQRT_3_ON_2,
    0.0,
    -0.5,
    SQRT_3_ON_2,
    0.0,
    0.5,
    SQRT_3_ON_2,
    -12.0,
    -0.5,
    SQRT_3_ON_2,
    -12.0,
    -0.5,
    SQRT_3_ON_2,
    0.0,
    -0.5,
    SQRT_3_ON_2,
    -12.0,
    -1.0,
    0.0,
    0.0,
    -1.0,
    0.0,
    0.0,
    -0.5,
    SQRT_3_ON_2,
    -12.0,
    -1.0,
    0.0,
    -12.0,
    -1.0,
    0.0,
    0.0,
    -1.0,
    0.0,
    -12.0,
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    -1.0,
    0.0,
    -12.0,
    -0.5,
    -SQRT_3_ON_2,
    -12.0,
    -0.5,
    -SQRT_3_ON_2,
    0.0,
    -0.5,
    -SQRT_3_ON_2,
    -12.0,
    0.5,
    -SQRT_3_ON_2,
    0.0,
    0.5,
    -SQRT_3_ON_2,
    0.0,
    -0.5,
    -SQRT_3_ON_2,
    -12.0,
    0.5,
    -SQRT_3_ON_2,
    -12.0,
    0.5,
    -SQRT_3_ON_2,
    0.0,
    0.5,
    -SQRT_3_ON_2,
    -12.0,
    1.0,
    0.0,
    0.0,
    1.0,
    0.0,
    0.0,
    0.5,
    -SQRT_3_ON_2,
    -12.0,
    1.0,
    0.0,
    -12.0,
];

pub const HEXAGONAL_PRISM_NORMALS: &[f32] = &[
    // Hexagon
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////////////////////
    // Prism: first face, northeast
    SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    // Prism: second face, north
    0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    // Prism: third face, northwest
    -SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    // Prism: fourth face, southwest
    -SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    -SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    // Prism: fifth face, south
    0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////////////////////
    // Prism: sixth face, southeast
    SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
    SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////////////////////
];

pub const TERRAIN_TRANSLATIONS: &[[f32; 3]] = &[
    [0.0, 0.0, 0.0],
    [-1.5, 0.8660254037844386, 0.0],
    [0.0, 1.7320508075688772, 0.0],
    [1.5, 0.8660254037844386, 0.0],
    [1.5, -0.8660254037844386, 0.0],
    [0.0, -1.7320508075688772, -1.0],
    [-1.5, -0.8660254037844386, 0.0],
    [-3.0, 1.7320508075688772, 0.0],
    [-1.5, 2.598076211353316, -1.0],
    [0.0, 3.4641016151377544, 1.0],
    [1.5, 2.598076211353316, 0.0],
    [3.0, 1.7320508075688772, 0.0],
    [3.0, 0.0, 1.0],
    [3.0, -1.7320508075688772, 1.0],
    [1.5, -2.598076211353316, 0.0],
    [0.0, -3.4641016151377544, -2.0],
    [-1.5, -2.598076211353316, 0.0],
    [-3.0, -1.7320508075688772, 1.0],
    [-3.0, 0.0, 1.0],
    [-4.5, 2.598076211353316, 0.0],
    [-3.0, 3.464101615137755, -2.0],
    [-1.5, 4.330127018922194, -2.0],
    [0.0, 5.196152422706632, 2.0],
    [1.5, 4.330127018922193, 1.0],
    [3.0, 3.4641016151377544, -1.0],
    [4.5, 2.598076211353316, -1.0],
    [4.5, 0.8660254037844388, 2.0],
    [4.5, -0.8660254037844384, 2.0],
    [4.5, -2.598076211353316, 0.0],
    [3.0, -3.464101615137755, 0.0],
    [1.5, -4.330127018922194, -3.0],
    [0.0, -5.196152422706632, -1.0],
    [-1.5, -4.330127018922193, -3.0],
    [-3.0, -3.4641016151377544, 1.0],
    [-4.5, -2.598076211353316, 2.0],
    [-4.5, -0.8660254037844388, 2.0],
    [-4.5, 0.8660254037844384, 0.0],
    [-6.0, 3.4641016151377544, 0.0],
    [-4.5, 4.330127018922193, -3.0],
    [-3.0, 5.196152422706632, -2.0],
    [-1.5, 6.06217782649107, 3.0],
    [0.0, 6.928203230275509, 3.0],
    [1.5, 6.06217782649107, 2.0],
    [3.0, 5.196152422706632, 0.0],
    [4.5, 4.330127018922193, -2.0],
    [6.0, 3.4641016151377544, -2.0],
    [6.0, 1.7320508075688772, -2.0],
    [6.0, 0.0, 3.0],
    [6.0, -1.7320508075688776, 3.0],
    [6.0, -3.4641016151377544, 1.0],
    [4.5, -4.330127018922193, 0.0],
    [3.0, -5.196152422706632, -4.0],
    [1.5, -6.06217782649107, -4.0],
    [0.0, -6.928203230275509, 0.0],
    [-1.5, -6.06217782649107, -1.0],
    [-3.0, -5.196152422706632, -4.0],
    [-4.5, -4.330127018922193, 2.0],
    [-6.0, -3.4641016151377544, 3.0],
    [-6.0, -1.7320508075688772, 3.0],
    [-6.0, 0.0, 3.0],
    [-6.0, 1.7320508075688776, 0.0],
    [-7.5, 4.330127018922193, 0.0],
    [-6.0, 5.196152422706631, -4.0],
    [-4.5, 6.0621778264910695, -2.0],
    [-3.0, 6.928203230275509, -1.0],
    [-1.5, 7.794228634059947, 4.0],
    [0.0, 8.660254037844386, 4.0],
    [1.5, 7.794228634059947, 3.0],
    [3.0, 6.928203230275509, 1.0],
    [4.5, 6.06217782649107, 1.0],
    [6.0, 5.196152422706632, -2.0],
    [7.5, 4.330127018922193, -3.0],
    [7.5, 2.5980762113533156, -3.0],
    [7.5, 0.8660254037844384, -3.0],
    [7.5, -0.8660254037844393, 3.0],
    [7.5, -2.598076211353316, 2.0],
    [7.5, -4.330127018922193, 1.0],
    [6.0, -5.196152422706631, 1.0],
    [4.5, -6.0621778264910695, 0.0],
    [3.0, -6.928203230275509, -5.0],
    [1.5, -7.794228634059947, -5.0],
    [0.0, -8.660254037844386, -1.0],
    [-1.5, -7.794228634059947, 1.0],
    [-3.0, -6.928203230275509, -1.0],
    [-4.5, -6.06217782649107, 3.0],
    [-6.0, -5.196152422706632, 3.0],
    [-7.5, -4.330127018922193, 2.0],
    [-7.5, -2.5980762113533156, 4.0],
    [-7.5, -0.8660254037844384, 4.0],
    [-7.5, 0.8660254037844393, 4.0],
    [-7.5, 2.598076211353316, 1.0],
    [-9.0, 5.196152422706632, 0.0],
    [-7.5, 6.062177826491071, 0.0],
    [-6.0, 6.92820323027551, -5.0],
    [-4.5, 7.794228634059948, -1.0],
    [-3.0, 8.660254037844387, 5.0],
    [-1.5, 9.526279441628825, 5.0],
    [0.0, 10.392304845413264, 5.0],
    [1.5, 9.526279441628825, 4.0],
    [3.0, 8.660254037844386, 2.0],
    [4.5, 7.794228634059948, 2.0],
    [6.0, 6.928203230275509, -2.0],
    [7.5, 6.0621778264910695, -2.0],
    [9.0, 5.196152422706632, -4.0],
    [9.0, 3.464101615137755, -2.0],
    [9.0, 1.7320508075688776, -4.0],
    [9.0, 0.0, 2.0],
    [9.0, -1.7320508075688767, 3.0],
    [9.0, -3.4641016151377535, 1.0],
    [9.0, -5.196152422706632, 2.0],
    [7.5, -6.062177826491071, 1.0],
    [6.0, -6.92820323027551, 1.0],
    [4.5, -7.794228634059948, -6.0],
    [3.0, -8.660254037844387, -6.0],
    [1.5, -9.526279441628825, -2.0],
    [0.0, -10.392304845413264, -2.0],
    [-1.5, -9.526279441628825, -2.0],
    [-3.0, -8.660254037844386, -1.0],
    [-4.5, -7.794228634059948, 4.0],
    [-6.0, -6.928203230275509, 4.0],
    [-7.5, -6.0621778264910695, 2.0],
    [-9.0, -5.196152422706632, 1.0],
    [-9.0, -3.464101615137755, 4.0],
    [-9.0, -1.7320508075688776, 5.0],
    [-9.0, 0.0, 4.0],
    [-9.0, 1.7320508075688767, 5.0],
    [-9.0, 3.4641016151377535, 2.0],
    [-10.5, 6.06217782649107, 0.0],
    [-9.0, 6.928203230275509, 0.0],
    [-7.5, 7.794228634059948, -6.0],
    [-6.0, 8.660254037844386, -6.0],
    [-4.5, 9.526279441628825, -1.0],
    [-3.0, 10.392304845413264, 6.0],
    [-1.5, 11.258330249197702, 5.0],
    [0.0, 12.12435565298214, 6.0],
    [1.5, 11.258330249197703, 3.0],
    [3.0, 10.392304845413262, 3.0],
    [4.5, 9.526279441628825, 3.0],
    [6.0, 8.660254037844386, 3.0],
    [7.5, 7.794228634059947, -3.0],
    [9.0, 6.928203230275509, -2.0],
    [10.5, 6.06217782649107, -5.0],
    [10.5, 4.330127018922193, -1.0],
    [10.5, 2.598076211353316, -5.0],
    [10.5, 0.8660254037844384, 1.0],
    [10.5, -0.8660254037844384, 1.0],
    [10.5, -2.598076211353315, 4.0],
    [10.5, -4.330127018922194, 1.0],
    [10.5, -6.06217782649107, 1.0],
    [9.0, -6.928203230275509, 3.0],
    [7.5, -7.794228634059948, 1.0],
    [6.0, -8.660254037844386, -7.0],
    [4.5, -9.526279441628825, -7.0],
    [3.0, -10.392304845413264, -3.0],
    [1.5, -11.258330249197702, -2.0],
    [0.0, -12.12435565298214, -3.0],
    [-1.5, -11.258330249197703, -3.0],
    [-3.0, -10.392304845413262, 0.0],
    [-4.5, -9.526279441628825, 5.0],
    [-6.0, -8.660254037844386, 5.0],
    [-7.5, -7.794228634059947, 5.0],
    [-9.0, -6.928203230275509, 0.0],
    [-10.5, -6.06217782649107, 2.0],
    [-10.5, -4.330127018922193, 0.0],
    [-10.5, -2.598076211353316, 3.0],
    [-10.5, -0.8660254037844384, 3.0],
    [-10.5, 0.8660254037844384, 6.0],
    [-10.5, 2.598076211353315, 4.0],
    [-10.5, 4.330127018922194, 3.0],
    [-12.0, 6.928203230275509, 1.0],
    [-10.5, 7.794228634059947, 0.0],
    [-9.0, 8.660254037844386, 0.0],
    [-7.5, 9.526279441628825, -7.0],
    [-6.0, 10.392304845413264, -7.0],
    [-4.5, 11.258330249197702, 5.0],
    [-3.0, 12.12435565298214, 7.0],
    [-1.5, 12.990381056766578, 7.0],
    [0.0, 13.856406460551018, 7.0],
    [1.5, 12.99038105676658, 2.0],
    [3.0, 12.12435565298214, 2.0],
    [4.5, 11.258330249197702, 2.0],
    [6.0, 10.392304845413264, 4.0],
    [7.5, 9.526279441628825, -3.0],
    [9.0, 8.660254037844386, -2.0],
    [10.5, 7.794228634059948, -6.0],
    [12.0, 6.928203230275509, -6.0],
    [12.0, 5.196152422706632, 0.0],
    [12.0, 3.4641016151377544, -6.0],
    [12.0, 1.7320508075688767, 1.0],
    [12.0, 0.0, 0.0],
    [12.0, -1.7320508075688767, 5.0],
    [12.0, -3.4641016151377553, 5.0],
    [12.0, -5.196152422706632, 2.0],
    [12.0, -6.928203230275509, 0.0],
    [10.5, -7.794228634059947, 0.0],
    [9.0, -8.660254037844386, 4.0],
    [7.5, -9.526279441628825, 1.0],
    [6.0, -10.392304845413264, -6.0],
    [4.5, -11.258330249197702, -4.0],
    [3.0, -12.12435565298214, -4.0],
    [1.5, -12.990381056766578, -1.0],
    [0.0, -13.856406460551018, -4.0],
    [-1.5, -12.99038105676658, -2.0],
    [-3.0, -12.12435565298214, -3.0],
    [-4.5, -11.258330249197702, 1.0],
    [-6.0, -10.392304845413264, 6.0],
    [-7.5, -9.526279441628825, 6.0],
    [-9.0, -8.660254037844386, 6.0],
    [-10.5, -7.794228634059948, 3.0],
    [-12.0, -6.928203230275509, 3.0],
    [-12.0, -5.196152422706632, 3.0],
    [-12.0, -3.4641016151377544, -1.0],
    [-12.0, -1.7320508075688767, 2.0],
    [-12.0, 0.0, 3.0],
    [-12.0, 1.7320508075688767, 7.0],
    [-12.0, 3.4641016151377553, 3.0],
    [-12.0, 5.196152422706632, 3.0],
    [-13.5, 7.794228634059947, 2.0],
    [-12.0, 8.660254037844386, -1.0],
    [-10.5, 9.526279441628823, 0.0],
    [-9.0, 10.392304845413262, -8.0],
    [-7.5, 11.258330249197702, -8.0],
    [-6.0, 12.124355652982139, 5.0],
    [-4.5, 12.990381056766578, 6.0],
    [-3.0, 13.856406460551018, 8.0],
    [-1.5, 14.722431864335455, 8.0],
    [0.0, 15.588457268119894, 8.0],
    [1.5, 14.722431864335457, 1.0],
    [3.0, 13.856406460551018, 1.0],
    [4.5, 12.99038105676658, 1.0],
    [6.0, 12.124355652982139, 5.0],
    [7.5, 11.258330249197702, 5.0],
    [9.0, 10.392304845413264, -4.0],
    [10.5, 9.526279441628825, -2.0],
    [12.0, 8.660254037844386, -7.0],
    [13.5, 7.794228634059947, -7.0],
    [13.5, 6.0621778264910695, -7.0],
    [13.5, 4.330127018922193, -7.0],
    [13.5, 2.598076211353315, 2.0],
    [13.5, 0.8660254037844384, -1.0],
    [13.5, -0.8660254037844384, -1.0],
    [13.5, -2.598076211353317, 6.0],
    [13.5, -4.330127018922194, 1.0],
    [13.5, -6.06217782649107, 2.0],
    [13.5, -7.794228634059947, -1.0],
    [12.0, -8.660254037844386, 0.0],
    [10.5, -9.526279441628823, -1.0],
    [9.0, -10.392304845413262, 1.0],
    [7.5, -11.258330249197702, -5.0],
    [6.0, -12.124355652982139, -5.0],
    [4.5, -12.990381056766578, -5.0],
    [3.0, -13.856406460551018, -5.0],
    [1.5, -14.722431864335455, -4.0],
    [0.0, -15.588457268119894, -4.0],
    [-1.5, -14.722431864335457, -5.0],
    [-3.0, -13.856406460551018, -3.0],
    [-4.5, -12.99038105676658, -2.0],
    [-6.0, -12.124355652982139, 6.0],
    [-7.5, -11.258330249197702, 5.0],
    [-9.0, -10.392304845413264, 7.0],
    [-10.5, -9.526279441628825, 7.0],
    [-12.0, -8.660254037844386, 4.0],
    [-13.5, -7.794228634059947, 4.0],
    [-13.5, -6.0621778264910695, 4.0],
    [-13.5, -4.330127018922193, 0.0],
    [-13.5, -2.598076211353315, -2.0],
    [-13.5, -0.8660254037844384, 3.0],
    [-13.5, 0.8660254037844384, 3.0],
    [-13.5, 2.598076211353317, 8.0],
    [-13.5, 4.330127018922194, 3.0],
    [-13.5, 6.06217782649107, 3.0],
    [-15.0, 8.660254037844386, 3.0],
    [-13.5, 9.526279441628823, 3.0],
    [-12.0, 10.392304845413262, -2.0],
    [-10.5, 11.258330249197702, 0.0],
    [-9.0, 12.124355652982139, -9.0],
    [-7.5, 12.990381056766578, 5.0],
    [-6.0, 13.856406460551018, 5.0],
    [-4.5, 14.722431864335455, 5.0],
    [-3.0, 15.588457268119894, 9.0],
    [-1.5, 16.454482671904334, 9.0],
    [0.0, 17.32050807568877, 9.0],
    [1.5, 16.454482671904334, 8.0],
    [3.0, 15.588457268119894, 0.0],
    [4.5, 14.722431864335457, 0.0],
    [6.0, 13.856406460551018, 6.0],
    [7.5, 12.990381056766578, 4.0],
    [9.0, 12.12435565298214, -5.0],
    [10.5, 11.258330249197702, -2.0],
    [12.0, 10.392304845413264, -8.0],
    [13.5, 9.526279441628825, -8.0],
    [15.0, 8.660254037844386, -8.0],
    [15.0, 6.928203230275509, -6.0],
    [15.0, 5.196152422706631, -8.0],
    [15.0, 3.4641016151377535, 3.0],
    [15.0, 1.7320508075688767, 2.0],
    [15.0, 0.0, -2.0],
    [15.0, -1.7320508075688785, 7.0],
    [15.0, -3.4641016151377553, 1.0],
    [15.0, -5.196152422706632, 1.0],
    [15.0, -6.928203230275509, 2.0],
    [15.0, -8.660254037844386, -2.0],
    [13.5, -9.526279441628823, 0.0],
    [12.0, -10.392304845413262, 0.0],
    [10.5, -11.258330249197702, 1.0],
    [9.0, -12.124355652982139, 1.0],
    [7.5, -12.990381056766578, -6.0],
    [6.0, -13.856406460551018, -6.0],
    [4.5, -14.722431864335455, -5.0],
    [3.0, -15.588457268119894, -5.0],
    [1.5, -16.454482671904334, -4.0],
    [0.0, -17.32050807568877, -4.0],
    [-1.5, -16.454482671904334, -4.0],
    [-3.0, -15.588457268119894, -6.0],
    [-4.5, -14.722431864335457, -3.0],
    [-6.0, -13.856406460551018, -1.0],
    [-7.5, -12.990381056766578, 6.0],
    [-9.0, -12.12435565298214, 8.0],
    [-10.5, -11.258330249197702, 8.0],
    [-12.0, -10.392304845413264, 6.0],
    [-13.5, -9.526279441628825, 5.0],
    [-15.0, -8.660254037844386, 5.0],
    [-15.0, -6.928203230275509, 5.0],
    [-15.0, -5.196152422706631, 1.0],
    [-15.0, -3.4641016151377535, -1.0],
    [-15.0, -1.7320508075688767, -3.0],
    [-15.0, 0.0, 3.0],
    [-15.0, 1.7320508075688785, 3.0],
    [-15.0, 3.4641016151377553, 9.0],
    [-15.0, 5.196152422706632, 3.0],
    [-15.0, 6.928203230275509, 3.0],
    [-16.5, 9.526279441628825, 4.0],
    [-15.0, 10.392304845413264, 4.0],
    [-13.5, 11.258330249197702, 2.0],
    [-12.0, 12.12435565298214, -3.0],
    [-10.5, 12.99038105676658, -10.0],
    [-9.0, 13.856406460551018, 5.0],
    [-7.5, 14.722431864335457, 4.0],
    [-6.0, 15.588457268119896, 4.0],
    [-4.5, 16.454482671904334, 4.0],
    [-3.0, 17.32050807568877, 10.0],
    [-1.5, 18.186533479473212, 10.0],
    [0.0, 19.05255888325765, 10.0],
    [1.5, 18.18653347947321, 10.0],
    [3.0, 17.32050807568877, -1.0],
    [4.5, 16.454482671904334, 1.0],
    [6.0, 15.588457268119896, 0.0],
    [7.5, 14.722431864335457, 7.0],
    [9.0, 13.856406460551018, -6.0],
    [10.5, 12.99038105676658, -2.0],
    [12.0, 12.12435565298214, -9.0],
    [13.5, 11.258330249197702, -7.0],
    [15.0, 10.392304845413262, -9.0],
    [16.5, 9.526279441628825, -9.0],
    [16.5, 7.794228634059948, -5.0],
    [16.5, 6.06217782649107, -5.0],
    [16.5, 4.330127018922193, 4.0],
    [16.5, 2.598076211353316, 2.0],
    [16.5, 0.8660254037844393, -3.0],
    [16.5, -0.8660254037844393, -3.0],
    [16.5, -2.598076211353316, 8.0],
    [16.5, -4.330127018922193, 2.0],
    [16.5, -6.0621778264910695, 0.0],
    [16.5, -7.794228634059946, 2.0],
    [16.5, -9.526279441628825, -1.0],
    [15.0, -10.392304845413264, -1.0],
    [13.5, -11.258330249197702, 0.0],
    [12.0, -12.12435565298214, 0.0],
    [10.5, -12.99038105676658, 1.0],
    [9.0, -13.856406460551018, -7.0],
    [7.5, -14.722431864335457, -6.0],
    [6.0, -15.588457268119896, -7.0],
    [4.5, -16.454482671904334, -5.0],
    [3.0, -17.32050807568877, -5.0],
    [1.5, -18.186533479473212, -4.0],
    [0.0, -19.05255888325765, -3.0],
    [-1.5, -18.18653347947321, -4.0],
    [-3.0, -17.32050807568877, -7.0],
    [-4.5, -16.454482671904334, -7.0],
    [-6.0, -15.588457268119896, -3.0],
    [-7.5, -14.722431864335457, 6.0],
    [-9.0, -13.856406460551018, 9.0],
    [-10.5, -12.99038105676658, 9.0],
    [-12.0, -12.12435565298214, 5.0],
    [-13.5, -11.258330249197702, 6.0],
    [-15.0, -10.392304845413262, 6.0],
    [-16.5, -9.526279441628825, 6.0],
    [-16.5, -7.794228634059948, 6.0],
    [-16.5, -6.06217782649107, 6.0],
    [-16.5, -4.330127018922193, 0.0],
    [-16.5, -2.598076211353316, -4.0],
    [-16.5, -0.8660254037844393, -4.0],
    [-16.5, 0.8660254037844393, 2.0],
    [-16.5, 2.598076211353316, 10.0],
    [-16.5, 4.330127018922193, 3.0],
    [-16.5, 6.0621778264910695, 3.0],
    [-16.5, 7.794228634059946, 4.0],
    [-18.0, 10.392304845413264, 3.0],
    [-16.5, 11.258330249197703, 5.0],
    [-15.0, 12.124355652982143, 5.0],
    [-13.5, 12.99038105676658, -4.0],
    [-12.0, 13.85640646055102, -11.0],
    [-10.5, 14.722431864335459, -11.0],
    [-9.0, 15.588457268119896, 6.0],
    [-7.5, 16.454482671904337, 3.0],
    [-6.0, 17.320508075688775, 3.0],
    [-4.5, 18.186533479473212, 11.0],
    [-3.0, 19.05255888325765, 9.0],
    [-1.5, 19.91858428704209, 9.0],
    [0.0, 20.784609690826528, 10.0],
    [1.5, 19.918584287042087, 11.0],
    [3.0, 19.05255888325765, 11.0],
    [4.5, 18.186533479473212, 2.0],
    [6.0, 17.32050807568877, 0.0],
    [7.5, 16.454482671904334, 8.0],
    [9.0, 15.588457268119896, 8.0],
    [10.5, 14.722431864335455, -2.0],
    [12.0, 13.856406460551018, -2.0],
    [13.5, 12.99038105676658, -10.0],
    [15.0, 12.124355652982139, -8.0],
    [16.5, 11.258330249197702, -10.0],
    [18.0, 10.392304845413264, -9.0],
    [18.0, 8.660254037844387, -9.0],
    [18.0, 6.92820323027551, -4.0],
    [18.0, 5.196152422706632, -4.0],
    [18.0, 3.4641016151377553, 5.0],
    [18.0, 1.7320508075688785, -4.0],
    [18.0, 0.0, -4.0],
    [18.0, -1.7320508075688767, -4.0],
    [18.0, -3.4641016151377535, 7.0],
    [18.0, -5.19615242270663, 3.0],
    [18.0, -6.928203230275507, 3.0],
    [18.0, -8.660254037844386, 0.0],
    [18.0, -10.392304845413264, -2.0],
    [16.5, -11.258330249197703, 0.0],
    [15.0, -12.124355652982143, 0.0],
    [13.5, -12.99038105676658, 1.0],
    [12.0, -13.85640646055102, 2.0],
    [10.5, -14.722431864335459, -8.0],
    [9.0, -15.588457268119896, -8.0],
    [7.5, -16.454482671904337, -7.0],
    [6.0, -17.320508075688775, -7.0],
    [4.5, -18.186533479473212, -5.0],
    [3.0, -19.05255888325765, -5.0],
    [1.5, -19.91858428704209, -3.0],
    [0.0, -20.784609690826528, -3.0],
    [-1.5, -19.918584287042087, -3.0],
    [-3.0, -19.05255888325765, -4.0],
    [-4.5, -18.186533479473212, -8.0],
    [-6.0, -17.32050807568877, -7.0],
    [-7.5, -16.454482671904334, -3.0],
    [-9.0, -15.588457268119896, 10.0],
    [-10.5, -14.722431864335455, 10.0],
    [-12.0, -13.856406460551018, 4.0],
    [-13.5, -12.99038105676658, 6.0],
    [-15.0, -12.124355652982139, 7.0],
    [-16.5, -11.258330249197702, 7.0],
    [-18.0, -10.392304845413264, 7.0],
    [-18.0, -8.660254037844387, 5.0],
    [-18.0, -6.92820323027551, 7.0],
    [-18.0, -5.196152422706632, 7.0],
    [-18.0, -3.4641016151377553, -1.0],
    [-18.0, -1.7320508075688785, -5.0],
    [-18.0, 0.0, 1.0],
    [-18.0, 1.7320508075688767, 11.0],
    [-18.0, 3.4641016151377535, 3.0],
    [-18.0, 5.19615242270663, 3.0],
    [-18.0, 6.928203230275507, 5.0],
    [-18.0, 8.660254037844386, 3.0],
];
