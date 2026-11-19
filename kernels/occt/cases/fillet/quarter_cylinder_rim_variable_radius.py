"""A quarter cylinder's curved rim rounded with a radius growing around it. A quarter of a cylinder,
its curved top rim rounded with a radius that grows from one end of the rim to the other.

The quarter cylinder: the part of the cylinder of radius 20 on the z axis, from z = 0 to z = 10,
with x ≥ 0 and y ≥ 0. It has two flat sides, in the planes y = 0 and x = 0, a quarter of a
cylindrical side, and quarter-disk top and bottom.

Round the edge where the top (z = 10) meets the curved side (ρ = 20): the quarter circle from (20,
0, 10) to (0, 20, 10). At the angle φ about the z axis, measured from +x toward +y, the radius is r
= 2 + 4φ/π: 2 where the edge meets the flat side y = 0 (φ = 0) and 4 where it meets the flat side x
= 0 (φ = 90°), growing evenly with the angle between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle
φ, the round's section is the quarter circle of radius r(φ) centred at (ρ, z) = (20 − r, 10 − r),
tangent to the top at ρ = 20 − r and to the side at z = 10 − r. The round meets each flat side in
such a quarter circle, of radius 2 on y = 0 and 4 on x = 0."""
import math

from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopExp import TopExp

from bench import Refused
from chamfer import edge_through

RADIUS, HEIGHT = 20, 10
RADIUS_AT_X, RADIUS_AT_Y = 2, 4  # the round's radius where the rim meets y = 0 and x = 0


def build():
    quarter = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT, math.pi / 2).Shape()
    s = RADIUS * math.sqrt(0.5)
    rim = edge_through(quarter, (s, s, HEIGHT))
    # A linear law from the edge's first vertex to its last: give each end its radius. On the
    # arc, linear in its parameter is linear in the angle.
    first = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(rim))
    fillet = BRepFilletAPI_MakeFillet(quarter)
    if first.X() > first.Y():  # starts on y = 0
        fillet.Add(RADIUS_AT_X, RADIUS_AT_Y, rim)
    else:
        fillet.Add(RADIUS_AT_Y, RADIUS_AT_X, rim)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the rim with a growing radius")
    return fillet.Shape()
