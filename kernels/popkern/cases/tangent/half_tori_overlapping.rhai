// The half of the torus about z (middle radius 10, tube 3) where z ≥ 0 united with the half where
// x ≥ 0.
let m = model();
let torus = |m| m.torus(10.0, 3.0);
let a = m.torus(10.0, 3.0).subtract(m.tool(|m| m.box(30.0, 30.0, 4.0, #{ at: [-15.0, -15.0, -4.0] })));
let b = m.tool(|m| m.torus(10.0, 3.0).subtract(m.tool(|m| m.box(15.0, 30.0, 8.0, #{ at: [-15.0, -15.0, -4.0] }))));
a = a.union(b);
m
