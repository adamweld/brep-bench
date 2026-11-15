// Two slabs united along z = 4, each with a flat B-spline patch there: the box (0, 0, 0)..(10, 10, 4)
// topped by a bicubic Bézier on x 0, 2, 7, 10 and y 0, 4, 5, 10, and the box (5, 3, 4)..(15, 13, 7)
// floored by a biquadratic one on x 5, 12, 15 and y 3, 4, 13. Each slab is a box standing past its
// patch, split by it.
let m = model();
let net = |xs, ys| {
    let n = [];
    for x in xs { let row = []; for y in ys { row.push([x, y, 4.0]); } n.push(row); }
    n
};
let top = m.sheet_nurbs(net.call([0.0, 2.0, 7.0, 10.0], [0.0, 4.0, 5.0, 10.0]), #{ degree: 3, label: "top" });
let bottom = m.sheet_nurbs(net.call([5.0, 12.0, 15.0], [3.0, 4.0, 13.0]), #{ degree: 2, label: "bottom" });
let lower = m.box(10.0, 10.0, 8.0).split([top], containing([5.0, 5.0, 1.0]))[0];
let upper = m.tool(|m| m.box(10.0, 10.0, 7.0, #{ at: [5.0, 3.0, 0.0] }).split([bottom], containing([10.0, 8.0, 6.0]))[0]);
lower = lower.union(upper);
m
