// Twelve balls of radius 3 on the circle of radius 10 about z, every 30 degrees from (10, 0, 0),
// joined one after another into a ring.
let m = model();
let pi = 3.141592653589793;
let ring = m.sphere(3.0).translate(10.0, 0.0, 0.0);
for k in 1..12 {
    let a = k * 30.0 * pi / 180.0;
    ring = ring.union(m.tool(|m| m.sphere(3.0).translate(10.0 * cos(a), 10.0 * sin(a), 0.0)));
}
m
