"""A flared dome's base moved out. A solid turned about the z axis. Its profile, in the half-plane x ≥ 0 (ρ = x, the distance from the
axis), runs from the axis at (0, 0) along z = 0 out to (5, 0); straight up and in to (3, 3); then
along the quarter ellipse (ρ/3)² + ((z − 3)/2)² = 1 up and in to the axis at (0, 5); and back down
the axis. Its faces are a flat base disk of radius 5 in z = 0, a conical flare from radius 5 at z =
0 to radius 3 at z = 3, and a dome that is half a spheroid of semi-axes 3 (across) and 2 (up), with
a seam.

Then edit the solid directly: move the base 1 outward along its normal, down to the plane z = −1.
The flare runs on down along its cone to meet it, so the base becomes a disk of radius 17/3. The
dome is unchanged.

So the result is the dome on a flare that runs from radius 17/3 at z = −1 to radius 3 at z = 3, over
a flat base disk."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import face_through
from edit import moved

def build():
    def line(a, b):
        return BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge()
    # the profile in the plane y = 0: the ellipse's centre (0, 0, 3), its major axis along x
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 3), gp_Dir(0, -1, 0), gp_Dir(1, 0, 0)), 3, 2)
    dome = BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge()  # (3, 0, 3) up to (0, 0, 5)
    wire = BRepBuilderAPI_MakeWire()
    for edge in (line((0, 0, 0), (5, 0, 0)), line((5, 0, 0), (3, 0, 3)), dome, line((0, 0, 5), (0, 0, 0))):
        wire.Add(edge)
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    solid = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))).Shape()
    base = face_through(solid, (1, 0, 0))
    return moved(solid, [(base, 1)])
