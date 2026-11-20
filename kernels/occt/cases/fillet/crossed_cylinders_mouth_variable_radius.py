"""The mouth of a cross hole through a cylinder rounded with a radius that varies round it. A cylinder of radius 8 on the z axis from z = −12 to z = 12, with a hole of radius 3 drilled right
through it along the x axis: the cylinder of radius 3 about the x axis, from x = −10 to x = 10, taken
away. The hole opens in the big cylinder's side at two mouths, one on the +x side and one on the −x
side.

Round the edge around the +x mouth, where the hole's wall meets the big cylinder's side, with a radius
that varies all the way round it: at the angle α about the x axis, measured from +y toward +z, the
edge's point is (√(64 − 9 cos²α), 3 cos α, 3 sin α) and r = 0.8 + 0.4 cos α. So the radius is 1.2 at
α = 0 (the point (√55, 3, 0)), 0.4 opposite at α = 180°, and 0.8 at the top and bottom of the mouth.
Leave the −x mouth sharp.

The round's sections square to the edge are circular: at each point of the edge, in the plane
through it square to the edge, the round's section is an arc of the circle of radius r(α) that lies
inside the solid and touches the hole's wall and the big cylinder's side in that plane. The edge's
angle inside the solid between the two walls is 90° where it crosses the plane y = 0 and smaller
elsewhere, down to 68° at α = 0 and at 180°."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where
from vround import rounded_with_law


def build():
    big = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -12), gp_Dir(0, 0, 1)), 8, 24).Shape()
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-10, 0, 0), gp_Dir(1, 0, 0)), 3, 20).Shape()
    cut = BRepAlgoAPI_Cut(big, hole)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not drill the hole")
    solid = cut.Shape()
    # The +x mouth: the edge pieces (the big cylinder's seam splits the loop in two) whose middle
    # point is 3 from the x axis on the +x side.
    mouth = edges_where(solid, lambda p: p.X() > 5 and abs(math.hypot(p.Y(), p.Z()) - 3) < 1e-6)
    return rounded_with_law(solid, mouth, lambda p: 0.8 + 0.4 * math.cos(math.atan2(p.Z(), p.Y())),
                            "the hole's +x mouth")
