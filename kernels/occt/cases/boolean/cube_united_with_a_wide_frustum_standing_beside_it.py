"""A cube united with a wide frustum standing beside it, cutting across its side. A cube of edge 6 from (0, 0, 0) to (6, 6, 6), and a cone frustum on the vertical axis through (3, −5), standing beside the cube's side face y = 0, 5 away from it: base radius 8 at z = 0, top radius 6 at z = 6, so its ends lie in the cube's bottom and top planes. Make their union. The frustum's radius is larger than the cube's edge at every height, and its wall cuts across a strip of the cube: it takes the whole face y = 0 and, from the faces x = 0 and x = 6, a strip bounded by the wall, √55 − 5 (about 2.4) deep at the bottom and 3√3 − 5 (about 0.2) deep at the top, measured from the face y = 0 at its edge. The frustum reaches no farther than y = 3. The result is one solid: the cube with the frustum's body covering its front.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(6, 6, 6)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(3, -5, 0), gp.DZ_s()), 8, 6, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
