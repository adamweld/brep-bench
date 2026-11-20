"""A quarter of a triangular ring, both ends moved out. A piece of a ring with a triangular section: the triangle in the half-plane y = 0, x > 0 with
corners at (x, z) = (3, 0), (5, 0) and (4, 2), turned about the z axis through a quarter turn. Both
its ends, the flat triangles in the planes y = 0 and x = 0, are moved out by 1 along their outward
normals."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import face_through
from edit import moved

MOVE = 1


def ring_quarter():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in ((3, 0), (5, 0), (4, 2)):
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp.OZ_s(), math.pi / 2).Shape()


def build():
    ring = ring_quarter()
    end_y = face_through(ring, (4, 0, 0.5))  # the end in the plane y = 0
    end_x = face_through(ring, (0, 4, 0.5))  # the end in the plane x = 0
    return moved(ring, [(end_y, MOVE), (end_x, MOVE)])
