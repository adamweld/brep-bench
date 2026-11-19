"""A box with rounded corners shelled open at the top, its wall centred on its faces. A box −20 ≤ x
≤ 20, −15 ≤ y ≤ 15, 0 ≤ z ≤ 20 with its four upright edges rounded to radius 5, shelled to a wall 2
thick, open at its top face, z = 20, with the wall centred on the faces it is grown from: half of
it, 1 thick, inside them, and half outside.

The moved faces are extended until they meet, every edge kept sharp, and run on up to the opening's
plane, z = 20, where the wall ends in a flat rim. Each flat side moves 1 out and 1 in; each rounded
corner becomes a quarter cylinder about the same axis, of radius 6 outside and 4 inside, still
tangent to the sides beside it; and the bottom moves to z = −1 outside and z = 1 inside.

So the solid is the rounded box 42 × 32 with corners of radius 6, from z = −1 to z = 20, less the
rounded box 38 × 28 with corners of radius 4, from z = 1 up."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.GeomAbs import GeomAbs_Intersection

from bench import Refused
from draft import rounded_box
from shell import face_through, shell

ROUND = 5
HALF = 1  # half the wall, each side of the faces


def build():
    box = rounded_box((-20, -15, 0), (20, 15, 20), ROUND)
    top = face_through(box, (0, 0, 20))
    # MakeThickSolid puts the wall on one side of the faces: the centred wall is the half outside
    # and the half inside, every edge kept sharp, joined.
    outside = shell(box, [top], HALF, outward=True, join=GeomAbs_Intersection)
    inside = shell(box, [top], HALF, join=GeomAbs_Intersection)
    wall = BRepAlgoAPI_Fuse(outside, inside)
    if not wall.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the wall's two halves")
    wall.SimplifyResult()
    return wall.Shape()
