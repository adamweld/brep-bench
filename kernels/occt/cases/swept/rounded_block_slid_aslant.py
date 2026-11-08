"""A block with every edge rounded, slid aslant. The solid a rounded block fills as it slides along
a straight line without turning.

The block is the box from (0, 0, 0) to (10, 6, 4) with every edge rounded to radius 1 and every
corner to a ball of radius 1: everything within 1 of the box from (1, 1, 1) to (9, 5, 3). Its faces
are six flat rectangles, twelve quarter cylinders and eight eighths of a ball.

It slides along the vector (8, 5, 3), ending as the same rounded block moved by (8, 5, 3). Make the
solid of everything the block passes through.

The rounded faces touch along their lines and arcs where their normals are square to the slide, and
those paths are what the flat sides of a box slid aslant would sweep, rounded: the result is
everything within 1 of the box from (1, 1, 1) to (9, 5, 3) slid along (8, 5, 3)."""
from OCP.BRepOffset import BRepOffset_Mode
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakeOffsetShape
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.GeomAbs import GeomAbs_JoinType
from OCP.gp import gp_Pnt

from bench import Refused
from sweep import slid, unified, united

CORE = ((1, 1, 1), (9, 5, 3))  # the box the block is everything within ROUND of
ROUND = 1
SLIDE = (8, 5, 3)


def build():
    # The rounded block is everything within 1 of the box from (1, 1, 1) to (9, 5, 3), so what it
    # passes through is everything within 1 of what that box passes through: a point is within 1
    # of the box at some instant exactly when it is within 1 of the box's sweep. The box's sweep
    # is its sweep's flat faces, as for `box-slid-aslant`: the box where it starts and its
    # leading faces slid, a convex polyhedron whose pieces only touch (glued), its faces merged
    # into its twelve planes. OCCT's offset with rounded joins then grows it by 1: its faces
    # moved out, its edges grown into cylinders and its corners into balls.
    core = BRepPrimAPI_MakeBox(gp_Pnt(*CORE[0]), gp_Pnt(*CORE[1])).Shape()
    swept = unified(united(slid(core, SLIDE), glue=True))
    grown = BRepOffsetAPI_MakeOffsetShape()
    grown.PerformByJoin(swept, ROUND, 1e-7, BRepOffset_Mode.BRepOffset_Skin, False, False,
                        GeomAbs_JoinType.GeomAbs_Arc)
    if not grown.IsDone():
        raise Refused("BRepOffsetAPI_MakeOffsetShape could not grow the box's sweep by 1")
    return grown.Shape()
