"""An ellipsoid with a concentric ellipsoid cut out of it. Semi-axes 6, 4, 3 less 5, 3, 2 on one
centre: a closed hollow ellipsoid."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_GTransform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_GTrsf, gp_Mat, gp_XYZ

from bench import Refused


def ellipsoid(a, b, c):
    # OCCT has no ellipsoid: the unit ball stretched by a non-uniform scale
    stretch = gp_GTrsf(gp_Mat(a, 0, 0, 0, b, 0, 0, 0, c), gp_XYZ(0, 0, 0))
    return BRepBuilderAPI_GTransform(BRepPrimAPI_MakeSphere(1).Shape(), stretch, True).Shape()


def build():
    cut = BRepAlgoAPI_Cut(ellipsoid(6, 4, 3), ellipsoid(5, 3, 2))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two ellipsoids")
    return cut.Shape()
