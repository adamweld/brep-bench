+++
title = "A parabolic-segment prism with its curved top edge rounded with a radius that varies along it"
tags = ["fillet", "variable-radius", "parabolic-cylinder", "planar", "unsettled"]
origin = "OCCT tests/blend/buildevol/C8, OCCT tests/blend/buildevol/C9"
settled = false
+++

The parabolic segment between the parabola y = 9 − x²/4 and its chord, the segment of the x axis
from (−6, 0) to (6, 0): the region 0 ≤ y ≤ 9 − x²/4, extruded from z = 0 to z = 4. It has a flat
bottom and top, a flat upright side in the plane y = 0, and a curved wall, the parabolic cylinder,
which meets the flat side at (±6, 0) at a slant (the parabola's slope there is ∓3). The parabola
curves tightest at its vertex (0, 9), with radius of curvature 2.

Round the one edge where the curved wall meets the top, the arc of the parabola at z = 4 from
(−6, 0, 4) to (6, 0, 4), with a radius that depends on where it is along the edge: r = 1 − x/12, so
3/2 where the edge meets the flat side at (−6, 0, 4), 1 at the vertex (0, 9, 4) and 1/2 at the other
end, (6, 0, 4). At each point of the edge the round's section, in the vertical plane through that
point square to the edge (the plane that contains the wall's horizontal normal there), is the quarter
circle of radius r tangent to the top and to the wall. The largest radius, 3/2, is less than the
parabola's least radius of curvature, 2, so the round does not fold. The round ends in the flat side y = 0, which cuts it obliquely: the normal sections meet y = 0 at a slant, so
the cut is not a quarter circle of radius 3/2 at one end or 1/2 at the other (it is about 1.57 and 0.53
wide). The flat side's own top edge stays sharp.

The round's volume and area have no closed form, so the case is unsettled until two kernels agree.
