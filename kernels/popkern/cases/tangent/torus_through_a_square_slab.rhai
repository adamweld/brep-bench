// A torus about z (middle radius 10, tube 3) united with the slab from (-13, -13, -1) to
// (13, 13, 1): its outer equator touches each side at one point.
let m = model();
let s = m.box(26.0, 26.0, 2.0, #{ at: [-13.0, -13.0, -1.0] });
s = s.union(m.tool(|m| m.torus(10.0, 3.0)));
m
