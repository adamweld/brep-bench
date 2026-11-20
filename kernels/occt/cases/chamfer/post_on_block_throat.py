"""A post on a block, its foot chamfered by the throat. A block from (−10, −10, 0) to (10, 10, 10),
with a post standing on its top: the cylinder of radius 3 about the z axis from z = 10 to z = 20,
joined to the block. The post's side meets the block's top in a concave circular edge, the circle
of radius 3 at z = 10.

Chamfer that edge by a throat of 1, as a weld is sized. A concave chamfer adds material: it fills
the right angle between the post and the block's top. In the plane through the z axis, the
chamfer's section is an isosceles right triangle: its two equal legs run from the edge along the
block's top and up the post's side, and its third side, the chamfer's face, lies 1 from the edge.
The legs are d = 1/cos 45° = √2, so the chamfer meets the top on the circle of radius 3 + √2 at
z = 10 and the post's side on the circle of radius 3 at z = 10 + √2. The chamfer is the cone
between those two circles, and the solid is the block and the post joined, with the ring between
them and the cone filled in."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.ChFiDS import ChFiDS_ChamfMode
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from chamfer import edge_through

THROAT = 1


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 10)).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 10), gp_Dir(0, 0, 1)), 3, 10).Shape()
    joined = BRepAlgoAPI_Fuse(block, post)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the block")
    joined = joined.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(joined)
    chamfer.SetMode(ChFiDS_ChamfMode.ChFiDS_ConstThroatChamfer)
    chamfer.Add(THROAT, edge_through(joined, (0, 3, 10)))
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the foot by its throat")
    return chamfer.Shape()
