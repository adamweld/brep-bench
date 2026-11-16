"""A ball cut flat above its middle, its rim chamfered until the flat is gone. The part of a ball of
radius 5 centered on the origin that lies below the plane z = 3. Its flat face is the disk of
radius 4 at z = 3. Chamfer the disk's rim with two distances: 4 on the flat face, the whole way
to its center, and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge). On the flat face that reaches the center (0, 0, 3), so the flat face
shrinks to a point. On the ball it reaches the circle whose points are 1 in a straight line from the
rim, the chord 1 long, turned 2·asin(1/10) about the ball's center down from the rim: radius ρ₁ =
98/25 + 9√11/50 ≈ 4.517 at z₁ = 147/50 − 6√11/25 ≈ 2.144. The chamfer is the cone from that circle
up to its apex at (0, 0, 3), and the solid is the cut ball cut again by it: the ball below z₁ under
that cone."""
from math import sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import cut_by_revolved


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    below = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, -20), 40, 40, 23).Shape()
    cut_ball = BRepAlgoAPI_Common(ball, below).Shape()
    # BRepFilletAPI_MakeChamfer declines a setback that consumes the flat face, so the solid is
    # built as the case states it: the cut ball cut again by the chamfer's cone. The cone runs
    # from its apex (0, 3) down through the circle on the ball 1 in a straight line from the rim,
    # (98/25 + 9√11/50, 147/50 − 6√11/25) in (ρ, z); what is cut away is everything above it.
    rho, z = 98 / 25 + 9 * sqrt(11) / 50, 147 / 50 - 6 * sqrt(11) / 25
    drop = 10 * (3 - z) / rho
    return cut_by_revolved(cut_ball, [(0, 3), (10, 3 - drop), (10, 6), (0, 6)],
                           "the cut ball by its chamfer's cone")
