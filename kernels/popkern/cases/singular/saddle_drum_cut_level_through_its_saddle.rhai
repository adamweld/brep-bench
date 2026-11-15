// The drum under the saddle z = 10 + xy/8 (a bilinear B-spline patch), cut level through its
// saddle point: everything above z = 10 cut away.
let m = model();
let saddle = m.sheet_nurbs([[[-10.0, -10.0, 22.5], [-10.0, 10.0, -2.5]], [[10.0, -10.0, -2.5], [10.0, 10.0, 22.5]]],
    #{ degree: 1, label: "saddle" });
let drum = m.cylinder(8.0, 30.0);
let parts = drum.split([saddle], containing([0.0, 0.0, 1.0]));
let d = parts[0];
d = d.subtract(m.tool(|m| m.box(20.0, 20.0, 20.0, #{ at: [-10.0, -10.0, 10.0] })));
m
