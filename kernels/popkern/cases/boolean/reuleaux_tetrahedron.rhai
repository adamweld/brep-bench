// The solid common to four balls of radius 10 centred on the vertices of a regular tetrahedron of
// side 10.
let m = model();
let s3 = sqrt(3.0);
let body = m.sphere(10.0);
for c in [[10.0, 0.0, 0.0], [5.0, 5.0 * s3, 0.0], [5.0, 5.0 * s3 / 3.0, 10.0 * sqrt(2.0 / 3.0)]] {
    body = body.intersect(m.tool(|m| m.sphere(10.0).translate(c[0], c[1], c[2])));
}
m
