// The upper half of the ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 (the sphere's net scaled by 6, 4 and 3),
// cut by a block below z = 0.
// The standard biquadratic sphere of radius `r` about `c`, scaled by `s` along each axis: the
// tensor of the rational quadratic circle (nine points, weights 1, √2/2, ...) and the rational
// quadratic semicircle from the south pole to the north (five points).
fn ball_net(c, r, s) {
    let w = sqrt(0.5);
    let ci = [[1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [-1.0, 1.0], [-1.0, 0.0], [-1.0, -1.0], [0.0, -1.0], [1.0, -1.0], [1.0, 0.0]];
    let cw = [1.0, w, 1.0, w, 1.0, w, 1.0, w, 1.0];
    let mer = [[0.0, -r], [r, -r], [r, 0.0], [r, r], [0.0, r]];
    let mw = [1.0, w, 1.0, w, 1.0];
    let net = [];
    let wts = [];
    for i in 0..9 {
        let row = [];
        let wr = [];
        for j in 0..5 {
            row.push([c[0] + s[0] * mer[j][0] * ci[i][0], c[1] + s[1] * mer[j][0] * ci[i][1], c[2] + s[2] * mer[j][1]]);
            wr.push(cw[i] * mw[j]);
        }
        net.push(row);
        wts.push(wr);
    }
    #{ net: net, weights: wts }
}
fn ball(m, c, r, s) {
    let b = ball_net(c, r, s);
    m.sheet_nurbs(b.net, #{ degree: [2, 2], weights: b.weights,
        knots: [[0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0]] })
}
let m = model();
let e = ball(m, [0.0, 0.0, 0.0], 1.0, [6.0, 4.0, 3.0]);
e = e.subtract(m.tool(|m| m.box(20.0, 20.0, 10.0, #{ at: [-10.0, -10.0, -10.0] })));
m
