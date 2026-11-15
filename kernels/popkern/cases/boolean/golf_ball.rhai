// A ball of radius 10 with 62 dimples, each cut by a ball of radius 1.2 centred 10.6 out toward a
// vertex, an edge's midpoint or a face's centre of the icosahedron (0, ±1, ±φ), (±1, ±φ, 0),
// (±φ, 0, ±1).
let m = model();
let phi = (1.0 + sqrt(5.0)) / 2.0;
let v = [];
for s in [1.0, -1.0] { for t in [1.0, -1.0] {
    v.push([0.0, s, t * phi]); v.push([s, t * phi, 0.0]); v.push([s * phi, 0.0, t]);
} }
fn dist2(a, b) { (a[0]-b[0])*(a[0]-b[0]) + (a[1]-b[1])*(a[1]-b[1]) + (a[2]-b[2])*(a[2]-b[2]) }
let dirs = [];
for p in v { dirs.push(p); }
// edges: vertex pairs 2 apart; faces: vertex triples pairwise 2 apart
let n = v.len();
for i in 0..n { for j in (i+1)..n {
    if abs(dist2(v[i], v[j]) - 4.0) < 1e-9 {
        dirs.push([(v[i][0]+v[j][0])/2.0, (v[i][1]+v[j][1])/2.0, (v[i][2]+v[j][2])/2.0]);
        for k in (j+1)..n {
            if abs(dist2(v[i], v[k]) - 4.0) < 1e-9 && abs(dist2(v[j], v[k]) - 4.0) < 1e-9 {
                dirs.push([(v[i][0]+v[j][0]+v[k][0])/3.0, (v[i][1]+v[j][1]+v[k][1])/3.0,
                    (v[i][2]+v[j][2]+v[k][2])/3.0]);
            }
        }
    }
} }
if dirs.len() != 62 { throw "expected 62 directions, found " + dirs.len(); }
let ball = m.sphere(10.0);
for d in dirs {
    let l = sqrt(d[0]*d[0] + d[1]*d[1] + d[2]*d[2]);
    let k = 10.6 / l;
    ball = ball.subtract(m.tool(|m| m.sphere(1.2).translate(d[0]*k, d[1]*k, d[2]*k)));
}
m
