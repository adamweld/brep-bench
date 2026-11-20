"""A spheroid dome's base rim rounded with a radius that varies all the way round it. A dome: the quarter of the ellipse (ρ/10)² + (z/5)² ≤ 1 with ρ ≥ 0 and z ≥ 0, turned once about the z
axis. It is a half spheroid, 5 high on a flat circular base of radius 10 in the plane z = 0, where
the spheroid stands square to the base.

Round the circular edge around the base, where the base meets the spheroid, with a radius that varies
all the way round it: at the angle φ about the z axis, measured from +x toward +y, r = 1.5 + 0.5 cos φ.
So the radius is 2 at φ = 0, where the edge's seam is, 1 opposite it, and varies smoothly between.

The round's sections square to the edge are circular: in the plane through the z axis at each angle φ,
the round's section is an arc of the circle of radius r(φ) that lies inside the dome and touches the
base and the ellipse: its center at height r, at the distance r inside the ellipse. (The ellipse's
radius of curvature at the base is 2.5, more than the largest radius.)"""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from chamfer import edge_through
from vround import angle_about_z, rounded_with_law


def build():
    # The quarter ellipse in the plane y = 0: its axes along x (10) and z (5), run from (10, 0, 0) to
    # (0, 0, 5), closed by the two radii along the axes.
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 10, 5)
    arc = BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge()
    axis = BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 5), gp_Pnt(0, 0, 0)).Edge()
    floor = BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(10, 0, 0)).Edge()
    wire = BRepBuilderAPI_MakeWire(arc)
    wire.Add(axis)
    wire.Add(floor)
    dome = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp.OZ_s()).Shape()
    rim = edge_through(dome, (0, 10, 0))  # the circle round the base
    return rounded_with_law(dome, rim, lambda p: 1.5 + 0.5 * math.cos(angle_about_z(p)),
                            "the dome's base rim")
