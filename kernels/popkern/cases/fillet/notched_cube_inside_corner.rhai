// A cube from the origin to (20, 20, 20) less the cube from (10, 10, 10) to (20, 20, 20); the
// notch's three concave edges rounded to 3 at once.
let m = model();
let c = m.box(20.0, 20.0, 20.0);
c = c.subtract(m.tool(|m| m.box(10.0, 10.0, 10.0, #{ at: [10.0, 10.0, 10.0] })));
c = c.fillet(edges().at_vertex([10.0, 10.0, 10.0]), 3.0, #{ label: "rounds" });
m
