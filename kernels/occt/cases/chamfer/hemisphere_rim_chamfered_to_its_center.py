"""A hemisphere whose rim is chamfered until its flat face is gone. The upper half of a ball of
radius 5 centered on the origin, z ≥ 0. Chamfer the circle where its flat face z = 0 meets the
ball, with two distances: 5 on the flat face, the whole way to its center, and 1 on the ball.

Each distance is measured in a straight line from the edge, in the plane through the z axis (the
plane square to the edge). On the flat face that reaches the center, so the flat face shrinks to a
point. On the ball it reaches the circle whose points are 1 in a straight line from the rim, the
chord 1 long, at radius 4.9 and height 3√11/10 ≈ 0.995. The chamfer is the cone from the origin up
to that circle, and the solid is the hemisphere cut by it: everything of the ball inside the cone, a
spherical sector whose half angle α from the z axis has sin α = 49/50."""
from math import sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import cut_by_revolved

ON_BALL = 1


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), 40, 40, 20).Shape()
    hemisphere = BRepAlgoAPI_Common(ball, upper).Shape()
    # BRepFilletAPI_MakeChamfer declines a setback that consumes the flat face, so the solid is
    # built as the case states it: the hemisphere cut by the chamfer's cone. The cone runs from
    # the origin through the circle on the ball the chord ON_BALL reaches, (4.9, 3√11/10) in
    # (ρ, z); what is cut away is everything outside it, down past the flat face.
    slope = (3 * sqrt(11) / 10) / 4.9
    return cut_by_revolved(hemisphere, [(0, 0), (10, 10 * slope), (10, -1), (0, -1)],
                           "the hemisphere by its chamfer's cone")
