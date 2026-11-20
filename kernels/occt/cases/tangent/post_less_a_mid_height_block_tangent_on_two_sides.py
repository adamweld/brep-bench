"""A post less a mid-height block tangent on two sides. A post of radius 5 on the z axis from z = 0 to
z = 12; a block x 0 to 7, y -5 to 5, z 4 to 8, its faces y = ±5 tangent to the post's wall. Cut the
block from the post."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(5, 12).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -5, 4), gp_Pnt(7, 5, 8)).Shape()
    cut = BRepAlgoAPI_Cut(post, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block tangent to the post")
    return cut.Shape()
