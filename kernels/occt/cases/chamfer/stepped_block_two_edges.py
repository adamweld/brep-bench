"""A stepped block made of two, refined, with a convex and a concave edge chamfered. A stepped block
made of two boxes joined, its coplanar faces merged, then two of its edges chamfered, one convex and
one concave.

The boxes: the base from the origin to (60, 40, 10), and on it the upper block from (0, 0, 10) to
(30, 40, 25), flush with the base at x = 0, y = 0 and y = 40. Join them, and merge each pair of
faces lying in one plane into one face: the faces x = 0, y = 0 and y = 40 each become one face, so
the solid's section in a plane y = c is the L through (x, z) = (0, 0), (60, 0), (60, 10), (30, 10),
(30, 25), (0, 25).

Then chamfer, each with two equal distances:

- the convex edge along y at (x, z) = (0, 25), where the merged face x = 0 meets the top z = 25, by
  5: its face runs from (0, 20) to (5, 25);
- the concave edge along y at (30, 10), where the step's riser x = 30 meets the base's top z = 10,
  by 3: a concave chamfer adds material, filling the corner up to the face from (33, 10) to (30,
  13).

Each chamfer runs the whole 40 along y and ends on the faces y = 0 and y = 40. The section becomes
the polygon (0, 0), (60, 0), (60, 10), (33, 10), (30, 13), (30, 25), (5, 25), (0, 20), and the solid
is that polygon extruded from y = 0 to y = 40."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.gp import gp_Pnt

from bench import Refused
from chamfer import edge_through


def build():
    base = BRepPrimAPI_MakeBox(60, 40, 10).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 10), gp_Pnt(30, 40, 25)).Shape()
    joined = BRepAlgoAPI_Fuse(base, upper)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the two boxes")
    # Refine: the faces in the planes x = 0, y = 0 and y = 40 become one face each.
    unify = ShapeUpgrade_UnifySameDomain(joined.Shape(), True, True, True)
    unify.Build()
    block = unify.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(block)
    chamfer.Add(5, edge_through(block, (0, 20, 25)))   # convex, top of the merged face x = 0
    chamfer.Add(3, edge_through(block, (30, 20, 10)))  # concave, foot of the step's riser
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the two edges")
    return chamfer.Shape()
