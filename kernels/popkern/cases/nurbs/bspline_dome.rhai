// A block on the square 0..20 by 0..20 under a clamped bicubic B-spline of 6 × 5 control points,
// knots (0, 0, 0, 0, 2/5, 2/5, 1, 1, 1, 1) and (0, 0, 0, 0, 1/3, 1, 1, 1, 1).
let m = model();
let xs = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
let ys = [0.0, 5.0, 10.0, 15.0, 20.0];
let z = [[3.0, 5.0, 6.0, 5.0, 4.0], [5.0, 9.0, 11.0, 8.0, 5.0], [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0], [5.0, 8.0, 9.0, 7.0, 4.0], [2.0, 4.0, 5.0, 4.0, 3.0]];
let net = [];
for i in 0..6 {
    let row = [];
    for j in 0..5 { row.push([xs[i], ys[j], z[i][j]]); }
    net.push(row);
}
let dome = m.sheet_nurbs(net, #{ degree: 3, knots: [[0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0],
    [0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, 1.0, 1.0, 1.0]], label: "dome" });
let block = m.box(20.0, 20.0, 30.0);
let b = block.split([dome], containing([10.0, 10.0, 1.0]))[0];
m
