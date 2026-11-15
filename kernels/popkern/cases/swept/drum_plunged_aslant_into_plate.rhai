// A plate from (0, -5, 0) to (20, 5, 4) less everything a drum (radius 2 about y = 0, z = -5,
// x = 6 to 14) passes through as it slides by 8 along (0.3, 0, √0.91).
//
// The drum is its end disc slid 8 along x; what it sweeps is that disc slid along the
// parallelogram P of x and the slide: P widened 2 either side in y, the drum where it starts and
// stops (P's edges along x), and its two end discs slid along the slide, oblique cylinders (P's
// edges along the slide), each a loft between two equal circles.
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
let s = sqrt(0.91);
let v = [2.4, 0.0, 8.0 * s];
let plate = m.box(20.0, 10.0, 4.0, #{ at: [0.0, -5.0, 0.0] });
plate = plate.subtract(m.tool(|m| zonotope(m, [10.0 + 1.2, 0.0, -5.0 + 4.0 * s], [[8.0, 0.0, 0.0], [0.0, 4.0, 0.0], v])));
for t in [0.0, 1.0] {
    plate = plate.subtract(m.tool(|m| m.cylinder(2.0, 8.0, #{ at: [6.0 + t * v[0], 0.0, -5.0 + t * v[2]], axis: [1.0, 0.0, 0.0], ref: [0.0, 1.0, 0.0] })));
}
for x in [6.0, 14.0] {
    plate = plate.subtract(m.tool(|m| {
        let a = m.profile(yz_at(x)).circle(2.0).at(0.0, -5.0).sketch("end");
        let b = m.profile(yz_at(x + v[0])).circle(2.0).at(0.0, -5.0 + v[2]).sketch("end_slid");
        m.loft([a, b], #{ label: "end_swept" })
    }));
}
m
