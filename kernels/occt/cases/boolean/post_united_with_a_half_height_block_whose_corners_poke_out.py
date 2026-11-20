"""A post united with a square block half as tall, its sides inside the post and its corners poking out. A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A block of square section, x and y from −4 to 4, and z from 0 to 5: it is half as tall as the post, its bottom face level with the post's base. Make the union of the post and the block. The block's side faces, x = ±4 and y = ±4, are at distance 4 from the axis and cross the post's wall only near their middles, where |y| < 3 (or |x| < 3); its corners, at distance 4√2 ≈ 5.66 from the axis, are outside the post. Four small corner pieces of the block stand out of the post's wall, up to z = 5, and above z = 5 the post stands whole."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(-4, -4, 0), gp_Pnt(4, 4, 5)).Shape()
    fuse = BRepAlgoAPI_Fuse(post, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
