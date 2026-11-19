"""A ball knob bored from below to its center. A ball knob, bored for the lever it will sit on: a
ball of radius 15 centred on the origin, with a round blind hole of radius 4 bored up into it along
the z axis from below.

The hole enters the ball through its lower side, around its south pole (0, 0, −15): its mouth is the
circle of radius 4 where the ball meets the cylinder ρ = 4, at z = −√209 ≈ −14.457. It runs up the z
axis and ends flat at the ball's centre, in the disk of radius 4 at z = 0. The ball's upper half is
whole.

If you cut the hole with a cylinder, make it run out past the ball below. Place the ball's poles on
the z axis: the south pole is in the hole and the north pole is on the ball's surface."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

BALL_RADIUS, HOLE_RADIUS = 15, 4
START = -20  # on the z axis, below the ball


def build():
    ball = BRepPrimAPI_MakeSphere(BALL_RADIUS).Shape()  # its poles on the z axis
    # The bore, from below the ball up to its centre. (BRepFeat_MakeCylindricalHole declines this
    # placement, BRepFeat_InvalidPlacement, while its axis runs through the ball's poles.)
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, START), gp_Dir(0, 0, 1)),
                                    HOLE_RADIUS, -START).Shape()
    cut = BRepAlgoAPI_Cut(ball, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not bore the knob")
    return cut.Shape()
