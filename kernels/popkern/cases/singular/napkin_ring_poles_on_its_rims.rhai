// A ball of radius 5 on the origin, its poles at (3, 0, 4) and (-3, 0, -4), bored through along z
// by a hole of radius 3: each rim runs through a pole.
let m = model();
let ring = m.sphere(5.0, #{ axis: [3.0, 0.0, 4.0] });
ring = ring.subtract(m.tool(|m| m.cylinder(3.0, 12.0, #{ at: [0.0, 0.0, -6.0] })));
m
