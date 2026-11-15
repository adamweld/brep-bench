// A ball of radius 5 on the origin, its poles on the line along (3.1, 0, 4), bored through along z
// by a hole of radius 3: each rim passes a little under a degree from a pole.
let m = model();
let ring = m.sphere(5.0, #{ axis: [3.1, 0.0, 4.0] });
ring = ring.subtract(m.tool(|m| m.cylinder(3.0, 12.0, #{ at: [0.0, 0.0, -6.0] })));
m
