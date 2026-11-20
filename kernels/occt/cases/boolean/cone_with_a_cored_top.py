"""A cone cored from the top. The frustum on the z axis from radius 6 at z = 0 to radius 4 at
z = 6, less a cutter made of a cylinder of radius 3 above z = 4 fused to a tube between radii 2
and 3 from z = 1 to z = 4."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def cylinder(radius, z0, z1):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, z0), gp.DZ_s()), radius, z1 - z0).Shape()


def build():
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 6, 4, 6).Shape()
    # The tube is a cylinder of radius 3 less one of radius 2, run past both its ends; the top
    # cylinder runs past the frustum's top, to z = 7.
    tube = BRepAlgoAPI_Cut(cylinder(3, 1, 4), cylinder(2, 0, 5))
    top = cylinder(3, 4, 7)
    cutter = BRepAlgoAPI_Fuse(top, tube.Shape())
    if not (tube.IsDone() and cutter.IsDone()):
        raise Refused("OCCT declined the cutter")
    cut = BRepAlgoAPI_Cut(frustum, cutter.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cutter")
    return cut.Shape()
