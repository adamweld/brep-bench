"""A flanged hub with six ribs and eight bolt holes, two rings patterned about its axis. A flanged
hub with six radial ribs and a bolt circle of eight holes: two rings of features, each one feature
patterned about the z axis.

- **The flange:** a disk of radius 50 on the z axis, from z = 0 to z = 8.
- **The hub:** a cylinder of radius 15 on the z axis, from z = 8 up to z = 40, joined to the flange.
- **The ribs:** one rib is the box 0 ≤ x ≤ 40, −2 ≤ y ≤ 2, 8 ≤ z ≤ 28, standing on the flange and
  running out from inside the hub; the part of it outside the hub is a plate 4 thick sticking out of
  the hub's side from ρ = 15 to its flat end at x = 40, 20 tall. It is patterned about the z axis
  every 60°: six ribs, the first along +x. All are joined to the hub and flange.
- **The bore:** a hole of radius 8 on the z axis, right through, from z = 0 to z = 40.
- **The bolt holes:** one hole of radius 3 on the vertical axis through (45, 0), right through the
  flange from z = 0 to z = 8, patterned about the z axis every 45°: eight holes, the first on the +x
  axis, so two of them (at 0° and 180°) lie just beyond the ends of ribs.

No bolt hole touches a rib (the ribs' ends reach ρ ≈ 40.05 at their corners, the holes start at ρ =
42), and the ribs meet each other only inside the hub."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused


def patterned(shape, count):
    """`shape` and its copies turned about the z axis, `count` of them in a whole turn."""
    copies = []
    for k in range(count):
        turn = gp_Trsf()
        turn.SetRotation(gp.OZ_s(), 2 * math.pi * k / count)
        copies.append(BRepBuilderAPI_Transform(shape, turn, True).Shape())
    return copies


def listed(shapes):
    found = List_TopoDS_Shape()
    for shape in shapes:
        found.Append(shape)
    return found


def build():
    flange = BRepPrimAPI_MakeCylinder(50, 8).Shape()
    hub = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 8), gp_Dir(0, 0, 1)), 15, 32).Shape()
    rib = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 8), gp_Pnt(40, 2, 28)).Shape()
    fuse = BRepAlgoAPI_Fuse()
    fuse.SetArguments(listed([flange]))
    fuse.SetTools(listed([hub] + patterned(rib, 6)))
    fuse.Build()
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the hub and the ribs to the flange")
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), 8, 42).Shape()
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(45, 0, -1), gp_Dir(0, 0, 1)), 3, 10).Shape()
    cut = BRepAlgoAPI_Cut()
    cut.SetArguments(listed([fuse.Shape()]))
    cut.SetTools(listed([bore] + patterned(hole, 8)))
    cut.Build()
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the bore and the bolt holes")
    return cut.Shape()
