"""A pipe lofted from a circle to a square, open at both ends. A solid lofted between two sections
along the z axis: the circle of radius 3 about the axis in the plane z = 0, and the boundary of the
square |x| ≤ 4, |y| ≤ 4 in the plane z = 10. The loft is ruled, and the sections are paired side by
side: the circle's quarter arc from 45° to 135° with the square's side y = 4, running from the
corner (4, 4) to the corner (−4, 4), and likewise the arcs from 135° to 225°, 225° to 315° and 315°
to 45° with the sides x = −4, y = −4 and x = 4, each running to the next corner. Within a pair the
points correspond in proportion: the point of the arc at 45° + 90°·s is joined by a straight line to
the point of the side at the fraction s of its length from its first corner, (4 − 8s, 4, 10), for s
from 0 to 1 (the point at 45° to the corner (4, 4), the point at 135° to (−4, 4)). So its side is
made of the lines so drawn, four ruled faces that meet in four sharp edges, the lines from the
circle's points at 45°, 135°, 225° and 315° to the square's corners. Its ends are the flat disk in z
= 0 and the flat square in z = 10.

Shell it inward to a wall 1/2 thick, open at both ends, a pipe. The outside of the wall is the
loft's side; the inside is the set of points of the loft at least 1/2 from that side, which runs on
from z = 0 to z = 10 to meet the planes of the openings, so the rims are flat in those planes. The
side is not developable and the inner surface is its parallel surface, with no closed form, so the
case is unsettled until two kernels agree."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused, face_through
from shell import shell


def build():
    circle = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 3)).Edge()
    square = BRepBuilderAPI_MakePolygon()
    for corner in ((4, -4), (4, 4), (-4, 4), (-4, -4)):
        square.Add(gp_Pnt(corner[0], corner[1], 10))
    square.Close()
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(BRepBuilderAPI_MakeWire(circle).Wire())
    loft.AddWire(square.Wire())
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the circle to the square")
    solid = loft.Shape()
    ends = [face_through(solid, (0.5, 0.5, 0)), face_through(solid, (0.5, 0.5, 10))]
    return shell(solid, ends, 0.5)
