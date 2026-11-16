"""A frustum capped by a ball, its rim chamfered by a distance of 2 and an angle of 45°. A frustum on the z axis, widening upward from the disk of radius 2 at z = 0 to radius 4 at z = 3, capped by the part of
the ball of radius 5 about the origin above z = 3: the ball meets the frustum's cone in its rim, the circle of radius 4
at z = 3, at about 70.5°, a convex edge.

Chamfer the rim by a distance and an angle: 2 down the cone from the rim, the chamfer making 45° with the cone's
side, turned toward the z axis. In the plane through the z axis the cone's side is the straight line from (2, 0) up to
the rim (4, 3). The chamfer starts on it at P, 2 from the rim, (2.891, 1.336) in (ρ, z), and is the straight line
from P that makes 45° with the cone's line continued toward the rim, turned toward the axis, until it meets the ball, 3.183
along, at Q ≈ (2.266, 4.457). The chamfer is the cone that line sweeps about the z axis, and the solid is the capped
frustum less the ring between the rim and that cone.

The angle is measured from the cone's side, which is straight in that plane, so it is the same angle everywhere along
the chamfer's start."""
from math import cos, radians, sin, sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeRevol, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Pnt

from bench import Refused

DOWN_CONE = 2
ANGLE = radians(45)


def build():
    above = BRepPrimAPI_MakeBox(gp_Pnt(-6, -6, 3), gp_Pnt(6, 6, 6)).Shape()
    cap = BRepAlgoAPI_Common(BRepPrimAPI_MakeSphere(5).Shape(), above).Shape()
    body = BRepAlgoAPI_Fuse(BRepPrimAPI_MakeCone(2, 4, 3).Shape(), cap).Shape()
    # BRepFilletAPI_MakeChamfer's distance-angle chamfer (AddDA) fits a B-spline to the bevel. The solid is
    # built here as the case states it: the ring between the rim, the chamfer's start P down the cone and the
    # chamfer's end Q on the ball cut away, as that sliver (with room outside the ball) turned about the axis.
    up = (2 / sqrt(13), 3 / sqrt(13))
    start = (4 - DOWN_CONE * up[0], 3 - DOWN_CONE * up[1])
    along = (up[0] * cos(ANGLE) - up[1] * sin(ANGLE), up[0] * sin(ANGLE) + up[1] * cos(ANGLE))
    toward = start[0] * along[0] + start[1] * along[1]
    root = sqrt(toward ** 2 - (start[0] ** 2 + start[1] ** 2 - 25))
    run = min(x for x in (-toward - root, -toward + root) if x > 0)
    end = (start[0] + run * along[0], start[1] + run * along[1])
    polygon = BRepBuilderAPI_MakePolygon()
    for rho, z in [start, (4, 3), (6, 4.5), (end[0] * 1.5, end[1] * 1.5), end]:
        polygon.Add(gp_Pnt(rho, 0, z))
    polygon.Close()
    ring = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face(), gp.OZ_s()).Shape()
    cut = BRepAlgoAPI_Cut(body, ring)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined to cut the rim by its chamfer's cone")
    return cut.Shape()
