"""A rib across a square pocket's diagonal, from corner to corner. A block from the origin to
(40, 40, 20) with a square pocket cut into its top: the box from (5, 5, 8) to (35, 35, 20) taken
away, so the pocket's floor is at z = 8 and its walls are the planes x = 5, x = 35, y = 5 and
y = 35.

In the pocket stands a rib along its diagonal, the line from (5, 5) to (35, 35), made from an open
profile in the vertical plane x = y through that diagonal: the two segments from (5, 5, 14) up to
(20, 20, 18) and down to (35, 35, 14), a ridge whose ends are on the pocket's corner edges. The rib
is that profile thickened 1 to each side of its plane, |x − y| ≤ √2, and carried down to the
pocket's floor: at each point of the floor within 1 of the diagonal plane it rises to the height of
the profile above that point's foot on the diagonal, z = h(s), where s = (x + y)/2 and

h = 14 + 4(1 − |s − 20|/15),

18 at the middle and 14 at the corners. At each end the rib runs into the pocket's corner, where the
walls x = 5 and y = 5 (and x = 35 and y = 35) cut it off; it fills the corner up to its two sloping
tops. The block and the rib are one solid."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepFeat import BRepFeat_MakeLinearForm
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.Geom import Geom_Plane
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt, gp_Vec

from bench import Refused

PROFILE = [(5, 5, 14), (20, 20, 18), (35, 35, 14)]  # in the plane x = y
HALF_THICKNESS = 1


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(40, 40, 20)).Shape()
    pocket = BRepPrimAPI_MakeBox(gp_Pnt(5, 5, 8), gp_Pnt(35, 35, 21)).Shape()
    cut = BRepAlgoAPI_Cut(block, pocket)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the pocket")
    part = cut.Shape()
    profile = BRepBuilderAPI_MakePolygon()
    for point in PROFILE:
        profile.Add(gp_Pnt(*point))
    across = gp_Dir(1, -1, 0)  # square to the rib's plane
    plane = Geom_Plane(gp_Pln(gp_Pnt(20, 20, 0), across))
    side = gp_Vec(across).Multiplied(HALF_THICKNESS)
    # A rib (fuse) from the open profile, thickened to both sides of its plane and carried down
    # to the faces it stands on.
    rib = BRepFeat_MakeLinearForm(part, profile.Wire(), plane, side, side.Reversed(), 1, True)
    rib.Perform()
    if not rib.IsDone():
        raise Refused("BRepFeat_MakeLinearForm could not make the rib")
    return rib.Shape()
