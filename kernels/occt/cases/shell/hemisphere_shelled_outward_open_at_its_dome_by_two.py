"""A half ball shelled outward by 2, open at its dome. The upper half of the ball of radius 5 centred on the origin: the part with z ≥ 0. Shell it outward to a wall 2 thick, open at its dome. The inside of the wall is the half ball's faces other than the opening, unmoved: its flat face, z = 0. The outside is that face moved 2 outward along its normal, onto the plane z = −2. The moved face is extended until it meets the surface of the opening, the sphere of radius 5, which is left as the rim between the outside and the inside.

So the result is the slice of the ball between z = −2 and z = 0: a disc 2 thick under the half ball, its edge the zone of the sphere between those planes, radius 5 at z = 0 down to √(25 − 2²) = 4.582575695 at z = −2. Nothing of the wall lies against the dome, which is open."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeHalfSpace, BRepPrimAPI_MakeSphere
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp, gp_Pln, gp_Pnt

from bench import Refused
from shell import face_through, shell

RADIUS = 5
THICKNESS = 2.0


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), RADIUS).Shape()
    plane = BRepBuilderAPI_MakeFace(gp_Pln(gp_Pnt(0, 0, 0), gp.DZ_s())).Face()
    below = BRepPrimAPI_MakeHalfSpace(plane, gp_Pnt(0, 0, -1)).Solid()
    cut = BRepAlgoAPI_Cut(ball, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the half-space")
    half = cut.Shape()
    # outward, the moved flat face extended until it meets the sphere
    return shell(half, [face_through(half, (3, 0, 4))], THICKNESS, outward=True,
                 join=GeomAbs_Intersection)
