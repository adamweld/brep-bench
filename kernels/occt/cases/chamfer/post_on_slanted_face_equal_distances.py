"""A post on a slanted face, the elliptical edge between them chamfered by equal distances. A block
from (−20, −20, −20) to (20, 20, 0), turned by 20° about the y axis, joined to the cylinder of
radius 4 on the z axis from z = −5 to z = 12; the edge where the post meets the slanted top is
chamfered 1 on each face: at each point P of the edge, in the plane square to the edge, the chamfer
meets each face at the point of it at straight-line distance 1 from P (on the post, the upper one)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused, face_through
from chamfer import edge_through


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, -20), gp_Pnt(20, 20, 0)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0)), math.radians(20))
    block = BRepBuilderAPI_Transform(block, turn).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -5), gp_Dir(0, 0, 1)), 4, 17).Shape()
    joined = BRepAlgoAPI_Fuse(block, post)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the block")
    joined = joined.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(joined)
    # the slanted top passes through the origin, z = −x tan 20°; the point (10, 0, ·) is on it
    top = face_through(joined, (10, 0, -10 * math.tan(math.radians(20))))
    chamfer.Add(1, 1, edge_through(joined, (0, 4, 0)), top)
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the elliptical edge")
    return chamfer.Shape()
