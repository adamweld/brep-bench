// A cross bar (radius 6 on x from -20 to 20) and a stem (radius 3 on z from 0 to 15) joined; the
// saddle-shaped junction rounded to 1.
let m = model();
let t = m.cylinder(6.0, 40.0, #{ at: [-20.0, 0.0, 0.0], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] });
t = t.union(m.tool(|m| m.cylinder(3.0, 15.0)));
t = t.fillet(edges().between("cylinder", "cylinder"), 1.0, #{ label: "junction" });
m
