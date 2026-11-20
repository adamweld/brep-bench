"""An elliptic ring: an ellipse swept round a circle. An ellipse swept once round a circle, with no ends: a ring of elliptic section.

The path is the circle of radius 12 about the z axis in the plane z = 0. The section, where the
path passes (12, 0, 0), is the ellipse in the plane y = 0 (square to the path there) centred on
(12, 0, 0), with semi-axis 3 along x (across the ring) and semi-axis 5 along z. The section
stays square to the circle and keeps its semi-axis of 3 along the radius from the z axis, so the
ring is the ellipse turned about the z axis."""
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
    path = BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), 12)).Edge()
    # the ellipse in the plane y = 0 about (12, 0, 0): its major axis (5) along z, its minor (3) along x
    section = BRepBuilderAPI_MakeEdge(
        gp_Elips(gp_Ax2(gp_Pnt(12, 0, 0), gp.DY_s(), gp.DZ_s()), 5, 3)).Edge()
    return pipe(path, section)
