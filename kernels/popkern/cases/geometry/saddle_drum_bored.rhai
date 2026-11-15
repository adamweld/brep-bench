// The drum under the saddle z = 10 + xy/8 (a bilinear B-spline patch), bored through along its
// axis by a hole of radius 3.
let m = model();
let saddle = m.sheet_nurbs([[[-10.0, -10.0, 22.5], [-10.0, 10.0, -2.5]], [[10.0, -10.0, -2.5], [10.0, 10.0, 22.5]]],
    #{ degree: 1, label: "saddle" });
let drum = m.cylinder(8.0, 30.0);
let parts = drum.split([saddle], containing([5.0, 0.0, 1.0]));
let d = parts[0];
d = d.subtract(m.tool(|m| m.cylinder(3.0, 32.0, #{ at: [0.0, 0.0, -1.0] })));
m
