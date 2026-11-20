"""A bent tube, one end moved in. The bent tube: the solid swept by a disk of radius 1.5 that moves along the cubic Bézier curve
with control points (0, 0, 0), (10, 0, 0), (10, 10, 5) and (20, 10, 5), always square to the
curve. At every point of the curve the disk lies in the plane square to it, centered on it. The
curve runs along +x at both ends, so the tube's end faces are the flat disks in the planes x = 0
and x = 20. The curve's radius of curvature is everywhere far more than 1.5, so the tube does not
overlap itself.

Then edit the solid directly: move the end in the plane x = 0 by 3 inward along its normal, +x, to
the plane x = 3. The tube is trimmed back to meet it, so that end is now the section of the tube
by x = 3, a flat face slightly out of square to the curve. The other end is untouched.

So the result is the part of the tube with x ≥ 3."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipe
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.Geom import Geom_BezierCurve
from OCP.GC import GC_MakeCircle
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import moved

RADIUS, DISTANCE, DEPTH = 1.5, 2, 3


def bent_tube():
    poles = Array1_gp_Pnt(1, 4)
    for i, p in enumerate([(0, 0, 0), (10, 0, 0), (10, 10, 5), (20, 10, 5)], start=1):
        poles.SetValue(i, gp_Pnt(*p))
    spine = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()).Wire()
    circle = GC_MakeCircle(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 0, 0)), RADIUS).Value()
    disk = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()).Face()
    return BRepOffsetAPI_MakePipe(spine, disk).Shape()


def build():
    tube = bent_tube()
    end = face_through(tube, (0, 0.5, 0))  # the end disk in x = 0, its outward normal −x
    return moved(tube, [(end, -DEPTH)])
