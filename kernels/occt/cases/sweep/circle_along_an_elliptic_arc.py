"""A circle swept along a quarter of an ellipse. A circle of radius 2 swept along a quarter of an ellipse. The path lies in the xy plane: the ellipse
with semi-axes 20 along x and 10 along y, centred on the origin, from (20, 0, 0) to (0, 10, 0)
(the points (20 cos t, 10 sin t, 0), t from 0 to a quarter turn). Its radius of curvature is least
at (20, 0, 0), 5, and greatest at (0, 10, 0), 40.

At every point of the path the section is the disc of radius 2 centred on it, in the plane square
to the path there. The pipe's two ends are flat discs, square to the path at its ends: in the plane
y = 0, and in the plane x = 0."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Elips, gp_Parab, gp_Pnt

from bench import Refused


def pipe(path_edge, section_edge):
    """The section swept along the path in the Frenet frame, closed with flat ends."""
    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path_edge).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps its orientation to the path
    pipe.Add(BRepBuilderAPI_MakeWire(section_edge).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the section")
    if not pipe.MakeSolid():
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()



def build():
    path = BRepBuilderAPI_MakeEdge(gp_Elips(gp.XOY_s(), 20, 10), 0, math.pi / 2).Edge()
    # the section at (20, 0, 0), square to the path, which leaves along +y
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(20, 0, 0), gp.DY_s()), 2)).Edge()
    return pipe(path, section)
