// A cube from (-5, -5, -5) to (5, 5, 5) united with a cylinder of radius 5 on the x axis from
// x = -10 to 10, tangent to four of its faces along their middle lines. Its seam is turned off
// those lines.
let m = model();
let c = m.box(10.0, 10.0, 10.0, #{ at: [-5.0, -5.0, -5.0] });
let s = sqrt(0.5);
c = c.union(m.tool(|m| m.cylinder(5.0, 20.0, #{ at: [-10.0, 0.0, 0.0], axis: [1.0, 0.0, 0.0], ref: [0.0, s, s] })));
m
