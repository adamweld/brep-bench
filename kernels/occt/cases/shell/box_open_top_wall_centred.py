"""A box shelled open at the top, its wall centred on its faces. The box from the origin to (10, 8, 6), shelled to a wall 2 thick, open at its top face, z = 6, with the wall centred on the faces it is grown from: half of it, 1 thick, inside them, and half outside. The moved faces are extended until they meet, every edge kept sharp, and run on up to z = 6, where the wall ends in a flat rim."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.GeomAbs import GeomAbs_Intersection

from bench import Refused
from shell import face_through, shell

HALF = 1  # half the wall, each side of the faces


def build():
    solid = BRepPrimAPI_MakeBox(10, 8, 6).Shape()
    openings = [face_through(solid, (5, 4, 6))]
    # MakeThickSolid puts the wall on one side of the faces: the centred wall is the half outside
    # and the half inside, every edge kept sharp, fused.
    outside = shell(solid, openings, HALF, outward=True, join=GeomAbs_Intersection)
    inside = shell(solid, openings, HALF, join=GeomAbs_Intersection)
    wall = BRepAlgoAPI_Fuse(outside, inside)
    if not wall.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the wall's two halves")
    wall.SimplifyResult()
    return wall.Shape()
