// A cube of side 10 from the origin, its footprint drawn in the top plane (x, y) and the prism
// over it, the three edges meeting at its corner (10, 10, 10) rounded to radius 2: where its top
// (`near`), its side x = 10 (swept from `east`) and its side y = 10 (from `north`) meet two by
// two, a ball's corner where the three rounds meet. `round` makes a ball's corner only where all
// three faces are on both its sides, so the three runs and the corner are taken by the library's
// own sets of them, each cut from the cube. Then the four faces deleted.
//
// Solvent has no word that takes a face out of a solid: a solid is the points of a set. The edit
// is said as one: deleting the rounds runs the faces round them on along their own surfaces until
// they meet, so what is left is the points on the material's side of the surface of each of the
// cube's six flat faces, read from the cube's own faces.
unit mm
use std
use std.solids
use std.rounds
use std.solids (cut)

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
}

cube := solid(std.solids.Prism(face(south, east, north, west), from: 0mm, to: 10mm))
top_east := solid(std.rounds.Run(cube.near, cube.east, r: 2mm))
top_north := solid(std.rounds.Run(cube.near, cube.north, r: 2mm))
east_north := solid(std.rounds.Run(cube.east, cube.north, r: 2mm))
corner := solid(std.rounds.Corner(cube.near, cube.east, cube.north, r: 2mm))
top_east cut cube
top_north cut cube
east_north cut cube
corner cut cube

part := solid({ p |
  p in cube.near.side
  p in cube.far.side
  p in cube.south.side
  p in cube.east.side
  p in cube.north.side
  p in cube.west.side
})
