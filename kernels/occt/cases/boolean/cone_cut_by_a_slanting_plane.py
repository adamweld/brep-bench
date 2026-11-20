"""A cone cut by a slanting plane. The cone with its apex at (0, 0, 12) and its base the disk of
radius 6 in z = 0, cut by the plane z = 4 + y/2, keeping the part above it, with the apex."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused


def build():
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 6, 0, 12).Shape()
    # Everything below the plane z = 4 + y/2: a big block, its top face on that plane.
    down = gp_Dir(0, 0.5, -1)
    along_x = gp_Dir(1, 0, 0)
    along_y = down.Crossed(along_x)
    origin = gp_Pnt(0, 0, 4).Translated(gp_Vec(along_x).Multiplied(-30)).Translated(
        gp_Vec(along_y).Multiplied(-30))
    below = BRepPrimAPI_MakeBox(gp_Ax2(origin, down, along_x), 60, 60, 30).Shape()
    cut = BRepAlgoAPI_Cut(cone, below)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slanting plane")
    return cut.Shape()
