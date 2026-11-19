"""A post with a collar open at its seam. The cylinder of radius 5 on the z axis from z = 0 to
z = 20, its seam in the half-plane φ = 0, united with a collar: the rectangle 5 ≤ ρ ≤ 8, 8 ≤ z ≤ 12
turned about the z axis from φ = 30° to φ = 330°, so that its bore lies on the post's wall and its
gap of 60° is centred on the post's seam."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import Refused

START = math.radians(30)


def at(rho, z):
    """The point at distance `rho` from the z axis and height `z` in the half-plane φ = 30°."""
    return gp_Pnt(rho * math.cos(START), rho * math.sin(START), z)


def build():
    post = BRepPrimAPI_MakeCylinder(5, 20).Shape()  # its seam at φ = 0
    section = BRepBuilderAPI_MakePolygon(at(5, 8), at(8, 8), at(8, 12), at(5, 12), True).Wire()
    collar = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(section).Face(), gp.OZ_s(),
                                   math.radians(300)).Shape()
    fuse = BRepAlgoAPI_Fuse(post, collar)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post and the collar")
    return fuse.Shape()
