"""A ruled elliptic loft united with an ellipsoid and an elliptic cylinder. Loft: ruled between the
ellipse in z = 0 (semi-axes 4 along x, 2 along y) and the one in z = 8 (2 along x, 4 along y), both
on the z axis, capped. Ellipsoid: semi-axes 5, 3, 4 about (2, 0, 4). Elliptic cylinder:
(x + 3)^2/9 + y^2/4 <= 1 from z = 2 to z = 6. United in that order."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_GTransform, BRepBuilderAPI_MakeEdge,
                                BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_GTrsf, gp_Mat, gp_Pnt, gp_Vec, gp_XYZ

from bench import Refused


def ellipse_wire(centre, major, minor):
    # major along x, minor along y, in a plane square to z
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(
        gp_Elips(gp_Ax2(gp_Pnt(*centre), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), major, minor)).Edge()).Wire()


def fused(a, b):
    fuse = BRepAlgoAPI_Fuse(a, b)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse failed")
    return fuse.Shape()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)
    loft.AddWire(ellipse_wire((0, 0, 0), 4, 2))
    loft.AddWire(ellipse_wire_turned())
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the ellipses")
    stretch = gp_GTrsf(gp_Mat(5, 0, 0, 0, 3, 0, 0, 0, 4), gp_XYZ(2, 0, 4))
    ellipsoid = BRepBuilderAPI_GTransform(BRepPrimAPI_MakeSphere(1).Shape(), stretch, True).Shape()
    face = BRepBuilderAPI_MakeFace(ellipse_wire((-3, 0, 2), 3, 2)).Face()
    cylinder = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, 4)).Shape()
    return fused(fused(loft.Shape(), ellipsoid), cylinder)


def ellipse_wire_turned():
    # the ellipse in z = 8 with semi-axis 2 along x and 4 along y, from the +y end of its long axis
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(
        gp_Elips(gp_Ax2(gp_Pnt(0, 0, 8), gp_Dir(0, 0, 1), gp_Dir(0, 1, 0)), 4, 2)).Edge()).Wire()
