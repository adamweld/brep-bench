// Two tori of middle radius 3 and tube 0.5, linked: one about z on the origin, one about a line
// parallel to y through (3, 0, 0). United, keeping both bodies.
let m = model();
let a = m.torus(3.0, 0.5);
let b = m.tool(|m| m.torus(3.0, 0.5).rotate(axis_at([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]), 90.0)
    .translate(3.0, 0.0, 0.0));
a = a.union(b);
m
