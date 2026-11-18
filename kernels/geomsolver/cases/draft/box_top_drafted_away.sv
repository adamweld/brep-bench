// The box from the origin to (30, 10, 10), its four sides drafted 30° with the pull along +z about
// the floor, would be its 30 × 10 footprint drawn in the top plane (x, y) and
// `solid(std.solids.Drafted(footprint, from: 0mm, to: 10mm, draft: 30deg))`.
//
// Refused: Solvent's draft (`std.solids.Drafted`) refuses a side its lean takes to nothing within
// the prism's ordinates (E080: "a draft of `30deg` takes the side `edge1` to nothing 8.66mm from
// the face's own plane, within the prism's ordinates (0mm to 10mm)"), so it cannot draft the box
// until its long sides meet in a ridge below the top and the top face is consumed. Stopping the
// prism at the ridge instead (`to: 5 * sqrt(3) * 1mm`) is no way round: the exact kernel then
// refuses the vanished top as "loop 0 has an edge of no length", and `5mm / tan(30deg)` lands a
// rounding short and is E080 again.
