"""A quarter-turn groove cut round a post that stands out of a box. A box from the origin to
(30, 30, 20), and on its side face x = 30 a post, a cylinder of radius 6 along +x about the line
y = 15, z = 10, from x = 30 to x = 45. A 4 × 4 square in the plane y = 15 (34 ≤ x ≤ 38, 4 to 8
above the axis) is revolved about the post's axis through 90°, right-handed, and cut away."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

AXIS = gp_Ax1(gp_Pnt(0, 15, 10), gp_Dir(1, 0, 0))


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(30, 30, 20)).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(30, 15, 10), gp_Dir(1, 0, 0)), 6, 15).Shape()
    fuse = BRepAlgoAPI_Fuse(box, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    outline = BRepBuilderAPI_MakePolygon()
    for x, dz in [(34, 4), (38, 4), (38, 8), (34, 8)]:
        outline.Add(gp_Pnt(x, 15, 10 + dz))
    outline.Close()
    sketch = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    groove = BRepPrimAPI_MakeRevol(sketch, AXIS, math.pi / 2).Shape()
    cut = BRepAlgoAPI_Cut(fuse.Shape(), groove)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
