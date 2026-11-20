"""A cylinder shelled open at the top, its wall centred on its faces, its edges kept sharp. A cylinder of radius 10 on the z axis, from z = 0 to z = 20, shelled to a wall 2 thick, open at its top face, with the wall centred on the faces it is grown from: half of it, 1 thick, inside the side and the bottom, and half outside them. The moved faces are extended until they meet, every edge kept sharp, and run on up to z = 20, where the wall ends in a flat rim."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_Intersection

from bench import Refused
from shell import face_through, shell

HALF = 1  # half the wall, each side of the faces


def build():
    solid = BRepPrimAPI_MakeCylinder(10, 20).Shape()
    openings = [face_through(solid, (0, 0, 20))]
    # MakeThickSolid puts the wall on one side of the faces: the centred wall is the half outside
    # and the half inside, every edge kept sharp, fused.
    outside = shell(solid, openings, HALF, outward=True, join=GeomAbs_Intersection)
    inside = shell(solid, openings, HALF, join=GeomAbs_Intersection)
    wall = BRepAlgoAPI_Fuse(outside, inside)
    if not wall.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the wall's two halves")
    wall.SimplifyResult()
    return wall.Shape()
