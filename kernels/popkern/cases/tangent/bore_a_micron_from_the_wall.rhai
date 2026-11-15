// A cylinder of radius 5 on z from 0 to 10, bored through by a hole of radius 4 on the line
// x = 0.999999: the wall at (5, 0) is a micron thick.
let m = model();
let c = m.cylinder(5.0, 10.0);
c = c.subtract(m.tool(|m| m.cylinder(4.0, 12.0, #{ at: [0.999999, 0.0, -1.0] })));
m
