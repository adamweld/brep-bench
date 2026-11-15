// A drum of radius 8 about z, its floor at z = 0 and its top the saddle z = 10 + xy/8: the
// bilinear B-spline patch on the corners of the square -10 ≤ x, y ≤ 10 at the saddle's heights.
// A drum standing past the saddle is split by it, and the piece below kept.
let m = model();
let saddle = m.sheet_nurbs([[[-10.0, -10.0, 22.5], [-10.0, 10.0, -2.5]], [[10.0, -10.0, -2.5], [10.0, 10.0, 22.5]]],
    #{ degree: 1, label: "saddle" });
let drum = m.cylinder(8.0, 30.0);
let parts = drum.split([saddle], containing([0.0, 0.0, 1.0]));
m
