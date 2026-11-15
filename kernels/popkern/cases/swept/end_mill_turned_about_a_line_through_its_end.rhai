// A block from (-10, -10, -5) to (10, 10, 0) less everything an end mill (radius 3 about the
// vertical through (1.5, 0), z = -2 to 4) passes through as it turns 120° about the z axis.
//
// A turn about z sweeps the mill where it starts and stops and, between them, its shadow on a
// half-plane through the axis revolved: the mill holds the axis and reaches 4.5 from it at every
// height, so its shadow is the rectangle 0 ≤ ρ ≤ 4.5, -2 ≤ z ≤ 4.
let m = model();
let b = m.box(20.0, 20.0, 5.0, #{ at: [-10.0, -10.0, -5.0] });
let pi = 3.141592653589793;
b = b.subtract(m.tool(|m| m.cylinder(3.0, 6.0, #{ at: [1.5, 0.0, -2.0] })));
b = b.subtract(m.tool(|m| m.profile(xz()).polygon([[0.0, -2.0], [4.5, -2.0], [4.5, 4.0], [0.0, 4.0]]).sketch("shadow")
    .revolve(axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]), blind(120.0))));
b = b.subtract(m.tool(|m| m.cylinder(3.0, 6.0, #{ at: [1.5 * cos(2.0 * pi / 3.0), 1.5 * sin(2.0 * pi / 3.0), -2.0] })));
m
