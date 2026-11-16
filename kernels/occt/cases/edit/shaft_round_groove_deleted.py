"""A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
groove turned into it (the torus whose tube, radius 1.5, runs round the circle of radius 5 at
z = 10, cut away), the groove's one face deleted: the shaft's cylinder below and above the groove
is one surface, which runs on across it.

The groove is cut away by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes its faces, picked
by a point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import cylinder, deleted


def build():
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 10), gp_Dir(0, 0, 1)), 5, 1.5).Shape()
    grooved = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 5, 20), ring).Shape()
    return deleted(grooved, [face_through(grooved, (0, -3.5, 10))])
