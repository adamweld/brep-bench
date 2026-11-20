"""A zigzag rib running out to an L-plate's sides. The L-plate is the rectangles 0 ≤ x ≤ 40,
0 ≤ y ≤ 20 and 0 ≤ x ≤ 20, 0 ≤ y ≤ 40, from z = 0 to 5. The rib is a wall 2 thick along the centre
line (0, 6) – (10, 6) – (10, 30) – (16, 30) – (16, 40), from z = 5 to z = 11, its ends flush with the
plate's sides x = 0 and y = 40, joined to the plate."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

PLATE = [(0, 0), (40, 0), (40, 20), (20, 20), (20, 40), (0, 40)]
# the rib's outline: the centre line offset 1 to each side, corners square
RIB = [(0, 5), (11, 5), (11, 29), (17, 29), (17, 40), (15, 40), (15, 31), (9, 31), (9, 7), (0, 7)]


def prism(points, z0, z1):
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in points:
        wire.Add(gp_Pnt(x, y, z0))
    wire.Close()
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, z1 - z0)).Shape()


def build():
    fuse = BRepAlgoAPI_Fuse(prism(PLATE, 0, 5), prism(RIB, 5, 11))
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the rib")
    return fuse.Shape()
