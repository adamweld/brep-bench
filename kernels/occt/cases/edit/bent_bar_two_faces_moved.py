"""A bent triangular bar, two of its faces moved out by 2. The right triangle with corners
(ρ, z) = (10, 0), (16, 0), (10, 9) swept a quarter turn about the z axis, from the plane y = 0 to
x = 0; its inner cylinder and its bottom each moved out by 2 along their outward normals."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import face_through
from edit import moved

PROFILE = [(10, 0), (16, 0), (10, 9)]  # (ρ, z)


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for rho, z in PROFILE:
        outline.Add(gp_Pnt(rho, 0, z))
    outline.Close()
    bar = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(outline.Wire()).Face(), gp.OZ_s(),
                                math.pi / 2).Shape()
    h = math.sqrt(0.5)
    inner = face_through(bar, (10 * h, 10 * h, 4.5))
    bottom = face_through(bar, (12 * h, 12 * h, 0))
    return moved(bar, [(inner, 2), (bottom, 2)])
