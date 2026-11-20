"""A square turned all the way round a boss on a box, a collar. A box from (−20, −20, 0) to
(20, 20, 10) with a round boss on its top, a cylinder of radius 6 about the z axis from z = 10 to
z = 22. A square sketch, 6 ≤ x ≤ 11 and 10 ≤ z ≤ 15 in the plane y = 0, is revolved a full turn
about the z axis and joined to the box and the boss: a collar on the box against the boss's wall."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

SQUARE = [(6, 10), (11, 10), (11, 15), (6, 15)]  # (x, z) in the plane y = 0


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), gp_Pnt(20, 20, 10)).Shape()
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 10), gp.DZ_s()), 6, 12).Shape()
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in SQUARE:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    sketch = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    collar = BRepPrimAPI_MakeRevol(sketch, gp.OZ_s()).Shape()  # a full turn
    part = box
    for piece in (boss, collar):
        fuse = BRepAlgoAPI_Fuse(part, piece)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a piece")
        part = fuse.Shape()
    return part
