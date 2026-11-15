// A ball of radius 5 on the origin, kept above the plane z = 4.9999: a cap 1e-4 high round its
// pole.
let m = model();
let b = m.sphere(5.0);
b = b.subtract(m.tool(|m| m.box(12.0, 12.0, 11.0, #{ at: [-6.0, -6.0, -6.0001] })));
m
