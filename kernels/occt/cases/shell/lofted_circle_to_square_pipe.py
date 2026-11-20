"""A pipe lofted from a circle to a square, open at both ends. A solid lofted between two sections
along the z axis: the circle of radius 3 about the axis in the plane z = 0, and the boundary of the
square |x| ≤ 4, |y| ≤ 4 in the plane z = 10. The loft is ruled: each point of the circle, (3 cos θ,
3 sin θ, 0), is joined by a straight line to the point of the square's boundary in the same
direction from the axis, (4 cos θ, 4 sin θ, 10)/max(|cos θ|, |sin θ|), for every angle θ. So its
side is made of the lines so drawn, four ruled faces that meet in four sharp edges, the lines from
the circle's points at 45°, 135°, 225° and 315° to the square's corners. Its ends are the flat disk
in z = 0 and the flat square in z = 10.

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
