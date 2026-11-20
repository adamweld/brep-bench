"""Three cylinders of radius 5 and length 20. The first lies along the x axis, from x = −10 to 10; the second along the y axis, from y = −10 to 10; the third along the z axis, from z = −10 to 10. Their axes are mutually perpendicular and meet at the origin. Make the solid common to all three.

The result is bounded by three bands of cylinder wall, one from each cylinder, each the part of its wall that lies inside the other two. Each pair of walls meets in two ellipses, in the planes x = ±y, y = ±z and x = ±z, and the solid's edges are arcs of them. Its eight corners, each where all three walls meet, are the points (\pm 5/\sqrt{2},\ \pm 5/\sqrt{2},\ \pm 5/\sqrt{2}).
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

RADIUS = 5
LENGTH = 20


def build():
    along_x = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-10, 0, 0), gp.DX_s()), RADIUS, LENGTH).Shape()
    along_y = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -10, 0), gp.DY_s()), RADIUS, LENGTH).Shape()
    along_z = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp.DZ_s()), RADIUS, LENGTH).Shape()
    first = BRepAlgoAPI_Common(along_x, along_y)
    if not first.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the first two cylinders")
    second = BRepAlgoAPI_Common(first.Shape(), along_z)
    if not second.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the third cylinder")
    return second.Shape()
