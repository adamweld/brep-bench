"""A circle swept along a parabola arc. A circle of radius 2 swept along an arc of a parabola. The arc lies in the xy plane: the curve
y = x²/16, from the origin to (16, 16, 0), whose radius of curvature is least at the origin, 8.

At every point of the arc the section is the disc of radius 2 centred on the arc, in the plane
square to the arc there. The pipe's two ends are flat discs, square to the arc at its ends: in the
plane x = 0, and in the plane through (16, 16, 0) square to the arc there."""
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
    # y = x²/16 is the parabola of focal length 4 whose axis is +y; its local Y runs along +x
    # from a frame turned the other way up.
    parabola = gp_Parab(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, -1), gp_Dir(0, 1, 0)), 4)
    path = BRepBuilderAPI_MakeEdge(parabola, 0, 16).Edge()
    # the section at the origin, square to the arc, which leaves along +x
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp.DX_s()), 2)).Edge()
    return pipe(path, section)
