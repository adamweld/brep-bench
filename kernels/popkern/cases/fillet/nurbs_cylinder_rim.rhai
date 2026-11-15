// A cylinder of radius 5 on z from 0 to 10 whose side is one NURBS surface (the rational quadratic
// circle times a line; sheet_nurbs closes it with flat discs), its top rim rounded to 1.
// The rational quadratic circle of radius `r` about the z axis (nine points, weights 1, √2/2, ...),
// starting at angle `a0`.
fn circle_net(r, z0, z1, quarter) {
    let w = sqrt(0.5);
    let ci = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [-1.0, 1.0], [-1.0, 0.0], [-1.0, -1.0], [0.0, -1.0], [1.0, -1.0], [1.0, 0.0]];
    let cw = [1.0, w, 1.0, w, 1.0, w, 1.0, w, 1.0];
    let net = [];
    let wts = [];
    for k in 0..9 {
        let p = ci[(k + 2 * quarter) % 8];
        net.push([[r * p[0], r * p[1], z0], [r * p[0], r * p[1], z1]]);
        wts.push([cw[k], cw[k]]);
    }
    #{ net: net, weights: wts }
}
fn nurbs_cylinder(m, r, z0, z1, quarter) {
    let c = circle_net(r, z0, z1, quarter);
    m.sheet_nurbs(c.net, #{ degree: [2, 1], weights: c.weights,
        knots: [[0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0], [0.0, 0.0, 1.0, 1.0]] })
}
let m = model();
let c = nurbs_cylinder(m, 5.0, 0.0, 10.0, 0);
c = c.fillet(edges().at_height(10.0), 1.0, #{ label: "rim" });
m
