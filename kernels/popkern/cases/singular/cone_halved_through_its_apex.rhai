// A cone on the disk of radius 5 at z = 0, apex (0, 0, 10), less everything where x < 0.
let m = model();
let cone = m.cone(5.0, 0.0, 10.0);
cone = cone.subtract(m.tool(|m| m.box(6.0, 12.0, 12.0, #{ at: [-6.0, -6.0, -1.0] })));
m
