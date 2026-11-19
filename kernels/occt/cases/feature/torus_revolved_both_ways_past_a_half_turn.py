"""A disk revolved both ways past a half turn: a whole torus. A disk revolved two ways at once about
the z axis, each way through more than half a turn, so that the two sweeps overlap and together
close into a whole torus.

The disk has radius 3 and centre (12, 0, 0), and lies in the plane y = 0. It is revolved about the z
axis through 200° turning from +x toward +y, and through 200° the other way, from +x toward −y. The
solid is everything the disk passes through in either sweep, one solid.

Each sweep runs past the half turn, so the two overlap between 160° and 200° round from the start
either way, and between them they cover the whole turn. The solid is the torus of major radius 12
and minor radius 3 about the z axis. No end face is left where either sweep stops, since each end
lies inside the other sweep."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

MAJOR, MINOR = 12, 3
SWEEP = 200  # degrees, each way


def build():
    circle = gp_Circ(gp_Ax2(gp_Pnt(MAJOR, 0, 0), gp_Dir(0, 1, 0)), MINOR)
    disk = BRepBuilderAPI_MakeFace(
        BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()).Face()
    # The two sides of a two-sided revolve: 200° about +z (towards +y), 200° about −z (towards −y).
    one = BRepPrimAPI_MakeRevol(disk, gp.OZ_s(), math.radians(SWEEP)).Shape()
    other = BRepPrimAPI_MakeRevol(disk, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, -1)),
                                  math.radians(SWEEP)).Shape()
    torus = BRepAlgoAPI_Fuse(one, other)
    if not torus.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the two overlapping sweeps")
    return torus.Shape()
