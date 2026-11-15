// Two balls of radius 5, on the origin and on (4.8, 0, 3.6), poles on the verticals through their
// centres: their union. The circle where they meet runs through a pole of each.
let m = model();
let a = m.sphere(5.0);
a = a.union(m.tool(|m| m.sphere(5.0, #{ at: [4.8, 0.0, 3.6] })));
m
