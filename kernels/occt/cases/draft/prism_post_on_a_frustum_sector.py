"""A triangular post on a sector of a frustum, its slanted side drafted. The quarter (x ≥ 0, y ≥ 0)
of the frustum about the z axis, radius 20 at z = 0 to 12 at z = 20, united with the prism over the
triangle (8, 4), (14, 4), (8, 10), from z = 0 to z = 26; its slanted face (through (14, 4) and
(8, 10)) drafted by 5°, pull direction +z, neutral plane z = 0, so that it leans toward the post's
inside as it rises."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import face_through
from draft import drafted, frustum_sector, united


def build():
    base = frustum_sector()
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in [(8, 4), (14, 4), (8, 10)]:
        wire.Add(gp_Pnt(x, y, 0))
    wire.Close()
    post = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp_Vec(0, 0, 26)).Shape()
    part = united(base, post)
    side = face_through(part, (11, 7, 24))
    return drafted(part, [side], 5, neutral=(0, 0, 0))
