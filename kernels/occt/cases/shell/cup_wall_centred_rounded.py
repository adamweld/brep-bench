"""A cup shelled with its wall centred on its faces, its outer rim rounded. A cylinder of radius 10
on the z axis, from z = 0 to z = 20, shelled to a wall 2 thick, open at its top face, z = 20, with
the wall centred on the faces it is grown from: half of it, 1 thick, inside the side and the bottom,
and half outside them.

At the bottom rim, the faces moved outward part, and are joined by a round: the wall's outside there
is the quarter torus of radius 1 about the rim circle. The faces moved inward cross, and are
extended until they meet in a sharp edge. The wall runs on up to the opening's plane, z = 20, where
it ends in a flat rim, the ring between radii 9 and 11.

So the solid is every point within 1 of the cylinder's side or bottom, cut off at z = 20: the
cylinder of radius 11 from z = −1 to z = 20 with its bottom rim rounded to radius 1, less the
cylinder of radius 9 from z = 1 up."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_Arc

from bench import Refused
from shell import face_through, shell

RADIUS, HEIGHT = 10, 20
HALF = 1  # half the wall, each side of the faces


def build():
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    top = face_through(cylinder, (0, 0, HEIGHT))
    # MakeThickSolid puts the wall on one side of the faces: the centred wall is the half outside
    # (its parted faces joined by a round at the rim) and the half inside, joined.
    outside = shell(cylinder, [top], HALF, outward=True, join=GeomAbs_Arc)
    inside = shell(cylinder, [top], HALF, join=GeomAbs_Arc)
    wall = BRepAlgoAPI_Fuse(outside, inside)
    if not wall.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the wall's two halves")
    wall.SimplifyResult()
    return wall.Shape()
