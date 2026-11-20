"""A post on a block, the concave edge between them chamfered with two distances. A block from
(−10, −10, 0) to (10, 10, 10), joined to a post, the cylinder of radius 4 on the z axis from z = 0
to z = 16. The post's side meets the block's top face in the circle of radius 4 at z = 10, a concave
edge. Chamfer that edge with two distances: 3/2 across the block's top face and 2 up the post. The
chamfer is the cone between the circles of radius 11/2 at z = 10 and radius 4 at z = 12."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused, face_through
from chamfer import edge_through


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 10)).Shape()
    post = BRepPrimAPI_MakeCylinder(4, 16).Shape()
    joined = BRepAlgoAPI_Fuse(block, post)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the block")
    joined = joined.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(joined)
    # 3/2 on the block's top face, 2 on the post
    chamfer.Add(1.5, 2, edge_through(joined, (0, 4, 10)), face_through(joined, (8, 8, 10)))
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the edge where the post meets the block")
    return chamfer.Shape()
