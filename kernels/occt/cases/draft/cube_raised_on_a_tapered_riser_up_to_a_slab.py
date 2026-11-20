r"""A cube raised on a tapered riser up to a slab. A cube with a tapered riser from its open side up to a slab: the extrusion of an open cube, drafted, until it meets the slab.

The cube is the box −5 ≤ x, y ≤ 5, 0 ≤ z ≤ 10. Its five faces are its floor z = 0 and four walls; the sixth side, the square at z = 10, is the opening, and it is the side that faces the slab, which lies above: the box −12 ≤ x, y ≤ 12, 16 ≤ z ≤ 18. (An extrusion up to the slab can only rise from the opening: the walls lie along the direction of rise, so extruding them that way sweeps out no volume.) The opening is extruded in the direction +z with a taper of 3°, each side of the riser leaning inward by 3° from the vertical as it rises, and stops at the slab's underside z = 16, where it joins the slab. The riser's side faces are four planes through the opening's four edges; at height z its section is the square |x|, |y| ≤ 5 − (z − 10)·tan 3°, so its top at z = 16 is a square of side 10 − 12·tan 3° ≈ 9.371. The cube, the riser and the slab are united into one solid: the cube's walls and floor are as they were, and the opening is no longer a face, being the riser's foot.

**As a set:**

$$\{|x|, |y| \le 5,\ 0 \le z \le 10\}\ \cup\ \{\,|x|, |y| \le 5 - (z - 10)\tan 3^\circ,\ 10 \le z \le 16\,\}\ \cup\ \{|x|, |y| \le 12,\ 16 \le z \le 18\}$$"""

import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFeat import BRepFeat_MakeDPrism
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Dir, gp_Pnt

from bench import Refused, face_through

TAPER = 3  # degrees


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 0), gp_Pnt(5, 5, 10)).Shape()
    slab = BRepPrimAPI_MakeBox(gp_Pnt(-12, -12, 16), gp_Pnt(12, 12, 18)).Shape()
    both = BRepAlgoAPI_Fuse(cube, slab).Shape()  # the two, apart
    opening = face_through(both, (0, 0, 10))
    underside = face_through(both, (0, 0, 16))
    # a drafted prism from the opening, fused on, up to the slab's underside; the sketch plane is
    # the opening's own, the draft leaning inward
    prism = BRepFeat_MakeDPrism(both, opening, opening, TAPER * math.pi / 180, True, True)
    prism.Perform(underside)
    prism.Build()
    if not prism.IsDone():
        raise Refused("BRepFeat_MakeDPrism could not draft the opening up to the slab")
    return prism.Shape()
