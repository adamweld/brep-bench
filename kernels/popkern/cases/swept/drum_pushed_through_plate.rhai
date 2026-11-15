// A plate from (0, -5, 0) to (20, 5, 4), less everything a drum (radius 2 about y = 0, z = -5,
// x = 6 to 14) passes through as it slides 16 straight up.
//
// Sliding square to its axis, the drum sweeps itself where it starts and stops and, between them,
// its section square to the slide (the rectangle 6 ≤ x ≤ 14, |y| ≤ 2) carried along it.
let m = model();
let plate = m.box(20.0, 10.0, 4.0, #{ at: [0.0, -5.0, 0.0] });
plate = plate.subtract(m.tool(|m| m.box(8.0, 4.0, 16.0, #{ at: [6.0, -2.0, -5.0] })));
plate = plate.subtract(m.tool(|m| m.cylinder(2.0, 8.0, #{ at: [6.0, 0.0, -5.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] })));
plate = plate.subtract(m.tool(|m| m.cylinder(2.0, 8.0, #{ at: [6.0, 0.0, 11.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] })));
m
