"""A cylinder cut aslant with its elliptical rim rounded with a radius that varies round it. A cylinder of radius 6 on the z axis from z = 0, its top cut by the plane z = 12 + 0.4x: it is 9.6
high at x = −6 and 14.4 high at x = 6, its top face an ellipse with the long axis 12.9 and the short
12. Round the elliptical edge around that top face with a radius that varies all the way round it: at
the angle φ about the z axis, measured from +x toward +y, r = 1.5 + 0.5 cos φ. So the radius is 2 on
the high side, 1 on the low side, and varies smoothly between.

The round's sections square to the edge are circular: at each point of the edge, in the plane through
it square to the edge, the round's section is an arc of the circle of radius r(φ) that lies inside the
solid and touches both faces' traces in that plane, the side's and the top's. The edge's angle between
the side and the top varies round it, from 68.2° on the high side to 111.8° on the low side."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeHalfSpace
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused
from chamfer import edge_through
from vround import angle_about_z, rounded_with_law


def build():
    cylinder = BRepPrimAPI_MakeCylinder(6, 30).Shape()
    # The half space above the plane z = 12 + 0.4x: the plane through (0, 0, 12) with normal
    # (−0.4, 0, 1), the side it points to the one removed.
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 12), gp_Dir(-0.4, 0, 1))).Face()
    above = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, 40)).Solid()
    cut = BRepAlgoAPI_Cut(cylinder, above)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the cylinder aslant")
    solid = cut.Shape()
    rim = edge_through(solid, (0, 6, 12))  # the ellipse, at its point on the plane x = 0
    return rounded_with_law(solid, rim, lambda p: 1.5 + 0.5 * math.cos(angle_about_z(p)),
                            "the slanted rim")
