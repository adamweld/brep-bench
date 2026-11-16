"""A bored ball whose rim is chamfered the whole length of its bore. A ball of radius 5 centered on
the origin, with a hole of radius 2 bored right through it along the z axis. The bore meets the
ball in two circles of radius 2, at z = ±√21. Chamfer the upper one with two distances: 2√21 down
the bore, its whole length, and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge). Down the bore that reaches the lower rim, so the bore shrinks to that
circle and is gone. On the ball it reaches the circle whose points are 1 in a straight line from the
upper rim, the chord 1 long, turned 2·asin(1/10) about the ball's center away from the bore: radius
ρ₁ = 49/25 + 3√231/50 ≈ 2.872 at z₁ = 49√21/50 − 3√11/25 ≈ 4.093. The chamfer is the cone from the
lower rim up to that circle, and the solid is the bored ball cut by it: the ball between z = −√21
and z₁, with a conical hole widening from radius 2 at the bottom to ρ₁ at the top."""
from math import sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from chamfer import cut_by_revolved


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    # The bore runs past both sides of the ball.
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp_Dir(0, 0, 1)), 2, 20).Shape()
    bored = BRepAlgoAPI_Cut(ball, bore).Shape()
    # BRepFilletAPI_MakeChamfer declines a setback that consumes the bore, so the solid is built
    # as the case states it: the bored ball cut by the chamfer's cone. The cone runs from the
    # lower rim (2, −√21) through the circle on the ball 1 in a straight line from the upper rim,
    # (49/25 + 3√231/50, 49√21/50 − 3√11/25) in (ρ, z); what is cut away is everything inside it.
    low, rho, z = -sqrt(21), 49 / 25 + 3 * sqrt(231) / 50, 49 * sqrt(21) / 50 - 3 * sqrt(11) / 25
    widening = (rho - 2) / (z - low)
    radius = lambda height: 2 + widening * (height - low)
    return cut_by_revolved(bored, [(0, -6), (radius(-6), -6), (radius(6), 6), (0, 6)],
                           "the bored ball by its chamfer's cone")
