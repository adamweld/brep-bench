// The solid everything within 1 of the box from (1, 1, 1) to (9, 5, 3) fills as it slides along
// (8, 5, 3): everything within 1 of what the box sweeps, the zonotope of its edges and the slide.
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
let z = zonotope(m, [5.0 + 4.0, 3.0 + 2.5, 2.0 + 1.5], [[8.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 2.0], [8.0, 5.0, 3.0]]);
z = z.offset_solid(1.0, #{ label: "rounded" });
m
