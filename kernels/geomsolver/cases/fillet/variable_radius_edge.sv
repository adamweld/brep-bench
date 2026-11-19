// A cube of side 10 from the origin, its footprint drawn in the top plane; the edge where its
// faces x = 10 (swept from `east`) and y = 10 (from `north`) meet rounded with a radius growing
// along it, r = 1 + z/10. `std.rounds` rounds by one radius, so the round is made as the case
// describes it, by its sections: at each station s up the edge (z = 10s), in the plane square to
// the edge there, the corner between the edge and the quarter circle of radius 1 + s tangent to
// both faces is a face; that face as s runs from 0 to 1 is the material the round takes, cut from
// the cube.
unit mm
use std
use std.solids
use std.solids (cut)
use std.stations
use std.place (coords)

// the corner the round takes at station `s` along the edge `g`, in the plane square to it: `u`
// runs along the face y = 10 toward −y and `v` along the face x = 10 toward −x, from the edge
component Taken(g: line, s: Scalar) {
  st := std.stations.OnLine(g, up: std.x, s: s)
  r := 1mm + 1mm * s
  in st.n {
    edge_pt := point hint((0, 0))
    edge_pt coords(du: 0mm, dv: 0mm) st.n
    on_x := point hint((1.5, 0))
    on_x coords(du: r, dv: 0mm) st.n
    on_y := point hint((0, -1.5))
    on_y coords(du: 0mm, dv: -r) st.n
    c := point hint((1.5, -1.5))
    c distance(r, along: u) st.n
    along_x := line(edge_pt, on_x)
    k := arc(center: c, start: on_x, end: on_y)
    along_y := line(on_y, edge_pt)
    f := face(along_x, k, along_y)
  }
}

in std.top {
  footprint := step(u: 10mm, v: 0mm) (south := line) -> step(v: 10mm) (east := line) ->
               step(u: 0mm) (north := line) -> step(v: 0mm) (west := line) -> close
}
foot := point
fix((10, 10, 0)) foot
head := point
fix((10, 10, 10)) head
construction edge := line(foot, head)

param s: Scalar over (0, 1)
round := solid({ p | some s: p in Taken(edge, s).f at s })
part := solid(std.solids.Prism(footprint, from: 0mm, to: 10mm))
round cut part
