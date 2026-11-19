// A cube of side 10 from the origin and a cylinder of radius 5 standing on the top plane from
// z = 0 to z = 10, its axis through the middle of the cube's face x = 10: the circle's centre is
// the midpoint of that face's footprint edge, and its radius half that edge, so the cylinder
// touches the faces y = 0 and y = 10 along the cube's edges there. The part is their union.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  a := point
  b := point hint((10, 0))
  c := point hint((10, 10))
  d := point hint((0, 10))
  horizontal (south := line(a, b)) -> vertical (east := line(b, c)) ->
  horizontal (north := line(c, d)) -> vertical (west := line(d, a)) -> close
  fix((0, 0)) a
  a distance(10) b
  b distance(10) c
  footprint := face(south, east, north, west)

  o := point hint((10, 5))
  o midpoint east
  end := circle(center: o) hint(r: 5)
  radius(5) end
}

boss := solid(std.solids.Prism(face(end), from: 0mm, to: 10mm))
part := solid(std.solids.Prism(footprint, from: 0mm, to: 10mm))
boss union part
