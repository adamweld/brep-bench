// Two tori about z of middle radius 5 and tube 2, their middle circles in z = -1 and z = 1: their
// common part.
let m = model();
let a = m.torus(5.0, 2.0).translate(0.0, 0.0, -1.0);
a = a.intersect(m.tool(|m| m.torus(5.0, 2.0).translate(0.0, 0.0, 1.0)));
m
