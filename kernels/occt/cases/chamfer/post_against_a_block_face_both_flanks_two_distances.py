"""A post standing out of a block's side, the two straight edges where it leaves the side chamfered
with two distances. In the xy plane, the rectangle from (−10, −10) to (10, 0) joined to the disc of
radius 5 centered at (0, 3), extruded along +z from 0 to 10. The edges (±4, 0, z) are chamfered 1
across the face y = 0 and 4/√5 on the post, a straight line from the edge in the plane z = const."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from chamfer import edge_through


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 0, 10)).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 3, 0), gp_Dir(0, 0, 1)), 5, 10).Shape()
    joined = BRepAlgoAPI_Fuse(block, post)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the block")
    joined = joined.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(joined)
    face = face_through(joined, (8, 0, 5))  # the block's face y = 0
    for x in (-4, 4):
        chamfer.Add(1, 4 / math.sqrt(5), edge_through(joined, (x, 0, 5)), face)
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the edges where the post leaves the block")
    return chamfer.Shape()
