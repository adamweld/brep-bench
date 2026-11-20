"""A cylinder less an equal cylinder raised half its height. Two cylinders of radius 5 on the z axis,
the first from z = 0 to z = 10 and the second from z = 5 to z = 15. Cut the second from the first.
Between z = 5 and z = 10 their walls coincide. What is left is the cylinder from z = 0 to z = 5:
its top is the plane z = 5 where the second's bottom was, and its wall is the first's wall below
that. If the cylinders you make have a seam, turn the second a quarter turn about the z axis, so
that the coincident walls are not split in the same place."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(5, 10).Shape()
    # OCCT's cylinder has its seam along its frame's x direction: this one's runs along y
    turned = gp_Ax2(gp_Pnt(0, 0, 5), gp.DZ_s(), gp.DY_s())
    second = BRepPrimAPI_MakeCylinder(turned, 5, 10).Shape()
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two cylinders")
    return cut.Shape()
