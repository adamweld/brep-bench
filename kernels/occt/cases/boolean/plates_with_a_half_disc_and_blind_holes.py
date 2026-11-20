"""A plate with a half-disk, a through hole, a quarter ring and a second plate with five blind
holes."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

HOLES = [(1.5, 0.3, 0.5), (3, 0.4, 1.0), (5, 0.5, 1.5), (7, 0.6, 1.8), (8.5, 0.5, 0.8)]


def cyl(x, y, z, radius, height):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, z), gp.DZ_s()), radius, height).Shape()


def op(kind, a, b, what):
    o = kind(a, b)
    if not o.IsDone():
        raise Refused(f"{kind.__name__} declined {what}")
    return o.Shape()


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 1)).Shape()
    # The half-disk: the half of a disk of radius 5 centred (10, 5) with x >= 10.
    disk = cyl(10, 5, 0, 5, 1)
    right = BRepPrimAPI_MakeBox(gp_Pnt(10, 0, -1), gp_Pnt(16, 10, 2)).Shape()
    half = op(BRepAlgoAPI_Common, disk, right, "the half-disk")
    solid = op(BRepAlgoAPI_Fuse, plate, half, "the half-disk")
    solid = op(BRepAlgoAPI_Cut, solid, cyl(5, 5, -1, 1, 3), "the hole")
    # The quarter ring: the annulus 2..4 about the origin, in the quadrant x, y >= 0.
    ring = op(BRepAlgoAPI_Cut, cyl(0, 0, 1, 4, 1), cyl(0, 0, 0, 2, 3), "the ring")
    quadrant = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(5, 5, 3)).Shape()
    ring = op(BRepAlgoAPI_Common, ring, quadrant, "the quarter ring")
    solid = op(BRepAlgoAPI_Fuse, solid, ring, "the ring")
    second = BRepPrimAPI_MakeBox(gp_Pnt(6.5, 0.5, 1), gp_Pnt(9.5, 9.5, 3)).Shape()
    for y, r, d in HOLES:
        # Each blind hole is a cylinder from its floor up past the top face (z = 3).
        second = op(BRepAlgoAPI_Cut, second, cyl(8, y, 3 - d, r, d + 1), "a blind hole")
    return op(BRepAlgoAPI_Fuse, solid, second, "the second plate")
