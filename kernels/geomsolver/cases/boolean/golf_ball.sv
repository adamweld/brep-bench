// A golf ball: a ball of radius 10 about the origin, and 62 balls of radius 1.2 cut from it, each
// its own cut, their centres 10.6 from the origin toward the 12 vertices, the 30 edges' midpoints
// and the 20 faces' centres of the icosahedron whose vertices are (0, ±1, ±φ), (±1, ±φ, 0) and
// (±φ, 0, ±1). The vertices are the case's list; its edges are the pairs of vertices 2 apart and
// its faces the triples of vertices each 2 apart, picked out of the list as it is elaborated. Each
// dimple's centre is said where it stands from the icosahedron: on the ray toward a vertex or an
// edge's midpoint, or equally far from a face's three corners, 10.6 from the origin.
unit mm
use std
use std.solids (cut)
use std.sets

phi := (1 + sqrt(5)) / 2
R := 10mm      // the ball
rd := 1.2mm    // a dimple's ball
D := 10.6mm    // from the origin to a dimple's centre
edge := 2mm    // the icosahedron's edge
verts := [(0mm, 1mm, phi * 1mm), (0mm, 1mm, -phi * 1mm), (0mm, -1mm, phi * 1mm),
          (0mm, -1mm, -phi * 1mm), (1mm, phi * 1mm, 0mm), (1mm, -phi * 1mm, 0mm),
          (-1mm, phi * 1mm, 0mm), (-1mm, -phi * 1mm, 0mm), (phi * 1mm, 0mm, 1mm),
          (phi * 1mm, 0mm, -1mm), (-phi * 1mm, 0mm, 1mm), (-phi * 1mm, 0mm, -1mm)]
k := 5.5       // a seed's stretch, from the unit icosahedron out to the dimples

o := point
fix((0, 0, 0)) o
part := solid(std.sets.Ball(o, r: R))

// toward each vertex
repeat a in verts {
  va := point
  fix((a.x, a.y, a.z)) va
  construction to_v := line(o, va)
  cv := point hint((k * a.x, k * a.y, k * a.z))
  cv coincident to_v
  o distance(D) cv
  dimple_v := solid(std.sets.Ball(cv, r: rd))
  dimple_v cut part
}

// toward each edge's midpoint
repeat a in verts as i {
  repeat b in verts as j {
    when i < j and abs(sqrt((a.x - b.x)^2 + (a.y - b.y)^2 + (a.z - b.z)^2) - edge) < 0.01mm {
      ea := point
      fix((a.x, a.y, a.z)) ea
      eb := point
      fix((b.x, b.y, b.z)) eb
      m := point
      m midpoint line(ea, eb)
      construction to_m := line(o, m)
      ce := point hint((k * (a.x + b.x) / 2, k * (a.y + b.y) / 2, k * (a.z + b.z) / 2))
      ce coincident to_m
      o distance(D) ce
      dimple_e := solid(std.sets.Ball(ce, r: rd))
      dimple_e cut part
    }
  }
}

// toward each face's centre: the point equally far from the face's three corners
repeat a in verts as i {
  repeat b in verts as j {
    repeat c in verts as l {
      when (i < j and j < l and
            abs(sqrt((a.x - b.x)^2 + (a.y - b.y)^2 + (a.z - b.z)^2) - edge) < 0.01mm and
            abs(sqrt((b.x - c.x)^2 + (b.y - c.y)^2 + (b.z - c.z)^2) - edge) < 0.01mm and
            abs(sqrt((a.x - c.x)^2 + (a.y - c.y)^2 + (a.z - c.z)^2) - edge) < 0.01mm) {
        fa := point
        fix((a.x, a.y, a.z)) fa
        fb := point
        fix((b.x, b.y, b.z)) fb
        fc := point
        fix((c.x, c.y, c.z)) fc
        cf := point hint((k * (a.x + b.x + c.x) / 3, k * (a.y + b.y + c.y) / 3,
                          k * (a.z + b.z + c.z) / 3))
        line(cf, fa) equal line(cf, fb)
        line(cf, fa) equal line(cf, fc)
        o distance(D) cf
        dimple_f := solid(std.sets.Ball(cf, r: rd))
        dimple_f cut part
      }
    }
  }
}
