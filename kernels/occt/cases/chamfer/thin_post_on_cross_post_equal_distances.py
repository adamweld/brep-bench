"""A thin post standing on a cross post, the saddle edge between them chamfered by equal distances.
The cylinder of radius 6 about the x axis from x = −15 to 15, joined to the cylinder of radius 3 on
the z axis from z = 0 to 12; the edge where the thin post leaves the thick one is chamfered 1 on
each."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from chamfer import edge_through


def build():
    thick = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-15, 0, 0), gp_Dir(1, 0, 0)), 6, 30).Shape()
    thin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 3, 12).Shape()
    joined = BRepAlgoAPI_Fuse(thick, thin)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the thin post to the thick one")
    joined = joined.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(joined)
    chamfer.Add(1, 1, edge_through(joined, (0, 3, math.sqrt(27))),
                face_through(joined, (10, 0, 6)))  # the thick post, along its top line
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the saddle edge")
    return chamfer.Shape()
