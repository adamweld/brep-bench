"""A groove turned inside a drum about a tilted axis, sealed in it. A solid drum with a closed
cavity inside it: a groove that stays inside the material all the way round, cut by turning a disk
part of a turn about a tilted axis.

The drum is a cylinder of radius 20 on the z axis, from z = −15 to z = 15.

The groove: let A be the line through the origin along (1/2, 0, √3/2), tilted 30° from the z axis
toward +x. The disk D has radius 3 and centre (0, 10, 0), and lies in the plane that holds A and
that centre, the plane z = √3·x. Turned about A (right-handed about the direction given), its centre
runs round a circle of radius 10 about A. The groove is everything the disk passes through between
the turns of 40° and 140° from where it lies, so it starts 40° round and sweeps 100°: a sector of
the torus of radii 10 and 3 about A, its two ends flat disks of radius 3 in the half-planes through
A at 40° and 140°. It is removed from the drum.

No part of the groove reaches the drum's faces: every point of it is within 13 of the origin. So the
result is the drum with a sealed cavity, a solid bounded by two shells, the drum's outside and the
cavity's wall."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire, BRepBuilderAPI_Transform)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused

DRUM_RADIUS, HALF_HEIGHT = 20, 15
AXIS = gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0.5, 0, math.sqrt(3) / 2))  # 30° from z towards +x
RING, SECTION = 10, 3
START, STOP = 40, 140  # degrees about the axis


def build():
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -HALF_HEIGHT), gp.DZ_s()),
                                    DRUM_RADIUS, 2 * HALF_HEIGHT).Shape()
    # The disk lies in the plane through the axis and its centre (0, 10, 0): its normal is the
    # axis crossed with the centre's direction.
    normal = AXIS.Direction().Crossed(gp_Dir(0, 1, 0))
    circle = gp_Circ(gp_Ax2(gp_Pnt(0, RING, 0), normal), SECTION)
    disk = BRepBuilderAPI_MakeFace(
        BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()).Face()
    # The groove starts 40° round from the disk and sweeps 100°.
    offset = gp_Trsf()
    offset.SetRotation(AXIS, math.radians(START))
    start = BRepBuilderAPI_Transform(disk, offset, True).Shape()
    groove = BRepPrimAPI_MakeRevol(start, AXIS, math.radians(STOP - START)).Shape()
    cut = BRepAlgoAPI_Cut(drum, groove)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not take the groove out of the drum")
    return cut.Shape()
