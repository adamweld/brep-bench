// The T-shaped block, its outline through (0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7,
// 4) and (0, 4) drawn in the top plane, would be `solid(std.solids.Drafted(outline, from: 0mm, to:
// 8mm, draft: 45deg))`, each side leaning 45° into the material as it rises.
//
// Refused: Solvent's draft (`std.solids.Drafted`) refuses a side its lean takes to nothing within
// the prism's ordinates (E080: "a draft of `45deg` takes the side `edge1` to nothing 2mm from the
// face's own plane, within the prism's ordinates (0mm to 8mm)"), so it cannot draft the block until
// its bar's sides close in a ridge at height 2 and its stem's at height 3, the top face consumed;
// no single far ordinate stops both ridges.
