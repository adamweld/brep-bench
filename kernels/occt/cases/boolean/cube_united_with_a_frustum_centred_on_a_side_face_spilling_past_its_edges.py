"""A cube united with a frustum centred on a side face, spilling past its edges. A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum standing on the plane z = 0 on the vertical axis through (3, 0), the middle of the cube's side face y = 0: base radius 4 at z = 0, top radius 2 at z = 6, so its ends lie in the cube's bottom and top planes. Make their union. Half of the frustum is inside the cube. Its base radius, 4, exceeds half the face's width, 3, so low down, below z = 3, its circle spills past the cube's side faces x = 0 and x = 6; above z = 3 the circle's radius is under 3 and it lies within the face's width. The result is the cube with a half cone against its side, which also reaches round to the faces x = 0 and x = 6 low down.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(6, 6, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(3, 0, 0), gp.DZ_s()), 4, 2, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
