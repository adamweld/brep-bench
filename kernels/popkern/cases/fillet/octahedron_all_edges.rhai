// The octahedron |x| + |y| + |z| ≤ 10, all twelve edges rounded to radius 2.
let m = model();
let o = m.pyramid(4, 10.0, 0.0, 10.0);
o = o.union(m.tool(|m| m.pyramid(4, 10.0, 0.0, 10.0, #{ axis: [0.0, 0.0, -1.0], ref: [1.0, 0.0, 0.0] })));
o = o.fillet(edges(), 2.0, #{ label: "rounds" });
m
