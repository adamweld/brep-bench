"""A block's top face turned 60° about a bottom edge and joined to it. A block from the origin to
(10, 20, 10), joined to the solid its top face sweeps when the face is turned 60° about the line of
the block's bottom edge at x = 0, z = 0 (the y axis), over toward −x.

The top face is the rectangle 0 ≤ x ≤ 10, 0 ≤ y ≤ 20 at z = 10. The turn is about the y axis, by 60°
in the sense that carries +z toward −x (a turn of −60° about +y), so the face lifts off the block
and tips over its edge x = 0.

In the plane y = 0 the face is the segment from (x, z) = (0, 10) to (10, 10). Its nearest point to
the line is its end (0, 10), 10 from it, and its farthest the end (10, 10), 10√2 from it. So the
sweep is bounded by the face where it starts (the block's top, which the two share), the face where
it stops, the cylinder of radius 10 about the y axis traced by the edge x = 0, z = 10, the cylinder
of radius 10√2 traced by the edge x = 10, z = 10, and its two ends in the planes y = 0 and y = 20,
flush with the block's.

Every point of the sweep lies above the block's top or beyond its side x = 0, so the sweep and the
block meet only in the top face, and the solid is the two joined there."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

from bench import Refused, face_through

SIZE = (10, 20, 10)
TURN = 60  # degrees


def build():
    block = BRepPrimAPI_MakeBox(*SIZE).Shape()
    top = face_through(block, (5, 10, 10))
    # About −y, so a positive turn carries +z towards −x: the face tips over the edge x = 0.
    edge = gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, -1, 0))
    sweep = BRepPrimAPI_MakeRevol(top, edge, math.radians(TURN)).Shape()
    joined = BRepAlgoAPI_Fuse(block, sweep)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the sweep to the block")
    joined.SimplifyResult()
    return joined.Shape()
