"""A prism with slanted sides shelled open at the top, its wall centred on its faces. The trapezoid
(0, 0), (12, 0), (9, 6), (3, 6) in the plane y = 0 extruded along y from 0 to 10, shelled to a wall 2
thick, open at its top face, z = 6, the wall centred on the faces it is grown from: half of it, 1
thick, inside them and half outside, every edge kept sharp."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from shell import face_through, shell

HALF = 1  # half the wall, each side of the faces


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in [(0, 0), (12, 0), (9, 6), (3, 6)]:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    solid = BRepPrimAPI_MakePrism(face, gp_Vec(0, 10, 0)).Shape()
    top = face_through(solid, (6, 5, 6))
    outside = shell(solid, [top], HALF, outward=True, join=GeomAbs_Intersection)
    inside = shell(solid, [top], HALF, join=GeomAbs_Intersection)
    wall = BRepAlgoAPI_Fuse(outside, inside)
    if not wall.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the wall's two halves")
    wall.SimplifyResult()
    return wall.Shape()
