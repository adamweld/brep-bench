"""A leaning wedge on a plate with rounded top edges, shelled inward through the plate's underside.
The plate from the origin to (10, 8, 3), its four top edges rounded to 1, the wedge on it (the prism
along y from y = 2 to 6 of the triangle (3, 3), (7, 3), (8, 9) in xz) united with it, and the whole
shelled 0.5 inward, open at z = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from blend import edges_where, rounded
from shell import face_through, shell


def build():
    plate = BRepPrimAPI_MakeBox(10, 8, 3).Shape()
    plate = rounded(plate, 1, edges_where(plate, lambda p: abs(p.Z() - 3) < 1e-9), "the plate's top edges")
    poly = BRepBuilderAPI_MakePolygon()
    for x, z in ((3, 3), (7, 3), (8, 9)):
        poly.Add(gp_Pnt(x, 2, z))
    poly.Close()
    face = BRepBuilderAPI_MakeFace(poly.Wire(), True).Face()
    wedge = BRepPrimAPI_MakePrism(face, gp_Vec(0, 4, 0)).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, wedge)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the plate and the wedge")
    solid = fuse.Shape()
    return shell(solid, [face_through(solid, (5, 4, 0))], 0.5)
