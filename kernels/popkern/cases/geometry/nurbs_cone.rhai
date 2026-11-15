// A cone on the disc of radius 5 at z = 0 with its apex at (0, 0, 10), its side one NURBS surface
// (the rational quadratic circle joined linearly to nine copies of the apex): sheet_nurbs closes it
// with the base disc.
let m = model();
let w = sqrt(0.5);
let ci = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [-1.0, 1.0], [-1.0, 0.0], [-1.0, -1.0], [0.0, -1.0], [1.0, -1.0], [1.0, 0.0]];
let cw = [1.0, w, 1.0, w, 1.0, w, 1.0, w, 1.0];
let net = [];
let wts = [];
for k in 0..9 {
    net.push([[5.0 * ci[k][0], 5.0 * ci[k][1], 0.0], [0.0, 0.0, 10.0]]);
    wts.push([cw[k], cw[k]]);
}
m.sheet_nurbs(net, #{ degree: [2, 1], weights: wts,
    knots: [[0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0], [0.0, 0.0, 1.0, 1.0]] });
m
