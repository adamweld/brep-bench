// The L-shaped block, its outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
// drawn in the top plane, would be `solid(std.solids.Drafted(outline, from: 0mm, to: 8mm, draft:
// 30deg))`, each side leaning 30° into the material as it rises.
//
// Refused: Solvent's draft (`std.solids.Drafted`) refuses a side its lean takes to nothing within
// the prism's ordinates (E080: "a draft of `30deg` takes the side `edge1` to nothing 4.33mm from
// the face's own plane, within the prism's ordinates (0mm to 8mm)"), so it cannot draft the block
// until the sides of each arm close in a ridge under the top and the top face is consumed.
