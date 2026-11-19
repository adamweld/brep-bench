"""A cylinder shelled inward, open at both ends: a tube. A cylinder of radius 10 on the z axis, from
z = 0 to z = 30, shelled inward to a wall 1.5 thick, open at both its flat ends.

The outside of the wall is the cylinder's side, unmoved, and the inside is the side moved 1.5
inward, the cylinder of radius 8.5 on the same axis. It runs on to both openings' planes, z = 0 and
z = 30, where the wall ends in flat rims. So the solid is a tube: everything between 8.5 and 10 from
the z axis with 0 ≤ z ≤ 30."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from shell import face_through, shell

RADIUS, HEIGHT = 10, 30
THICKNESS = 1.5


def build():
    cylinder = BRepPrimAPI_MakeCylinder(RADIUS, HEIGHT).Shape()
    ends = [face_through(cylinder, (0, 0, 0)), face_through(cylinder, (0, 0, HEIGHT))]
    return shell(cylinder, ends, THICKNESS)
