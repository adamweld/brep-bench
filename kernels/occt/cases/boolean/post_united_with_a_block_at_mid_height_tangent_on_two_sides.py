"""A post united with a block at mid-height, two of its faces tangent to the post. A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A block with x from −5 to 5, y from −3 to 3 and z from 3 to 7: it lies at the middle of the post's height and reaches neither end. Make the union of the post and the block. The block's faces x = ±5 each touch the post's wall along a vertical line (x = ±5, y = 0, from z = 3 to z = 7), where block and post are tangent. Its faces y = ±3 cross the post's wall at x = ±4. The wall inside the block is covered by it, two curved strips on the post's two sides, and the block's corners stand out of the post."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-5, -3, 3), gp_Pnt(5, 3, 7)).Shape()
    fuse = BRepAlgoAPI_Fuse(post, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
