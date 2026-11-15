// A plate from (-15, -15, 0) to (15, 15, 4) less everything a tilted box (4 by 3 by 2 along
// e1 = (2, 1, 2)/3, e2 = (-2, 2, 1)/3, e3 = (1, 2, -2)/3, centred at (-2, -1, -8)) passes through
// as it slides along (4, 2, 18), wholly through the plate.
//
// What the box sweeps is the zonotope of its three edges and the slide, centred halfway along it.
fn cross(a, b) { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn dot(a, b) { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
// The zonotope centred at `c` with generators `g` (the sum of the segments from -g/2 to g/2): the
// solid common to one slab for each pair of generators, square to both, as thick as the
// generators reach across it. A box slid along a vector is one.
fn zonotope(m, c, g) {
    let body = ();
    let first = true;
    for i in 0..g.len() { for j in (i + 1)..g.len() {
        let n = cross(g[i], g[j]);
        let l = sqrt(dot(n, n));
        if l < 1e-9 { continue; }
        n = [n[0] / l, n[1] / l, n[2] / l];
        let reach = 0.0;
        for k in g { reach += abs(dot(k, n)); }
        let r = cross(n, if abs(n[0]) < 0.9 { [1.0, 0.0, 0.0] } else { [0.0, 1.0, 0.0] });
        let slab = m.box(500.0, 500.0, reach, #{ at: c, axis: n, ref: r, centred: true });
        if first { body = slab; first = false; } else { body = body.intersect(m.tool(|m| slab)); }
    } }
    body
}
let m = model();
let e1 = [2.0 / 3.0, 1.0 / 3.0, 2.0 / 3.0];
let e2 = [-2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0];
let e3 = [1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0];
let g = [[4.0 * e1[0], 4.0 * e1[1], 4.0 * e1[2]], [3.0 * e2[0], 3.0 * e2[1], 3.0 * e2[2]],
    [2.0 * e3[0], 2.0 * e3[1], 2.0 * e3[2]], [4.0, 2.0, 18.0]];
let swept = m.tool(|m| zonotope(m, [0.0, 0.0, 1.0], g));
let plate = m.box(30.0, 30.0, 4.0, #{ at: [-15.0, -15.0, 0.0] });
plate = plate.subtract(swept);
m
