"""A twisted square frustum shelled with its narrow top open. The solid made by a ruled loft between
the square of side 4 centered (0, 0, 8), corners (±2, ±2, 8), and the square of side 8 in the plane
z = 0 centered (1, 0, 0) and turned 30° about its center, corners joined by their signs, closed by
the two squares, shelled inward to a wall 0.5 thick, open at its top face."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused
from shell import shell_to_plane

HEIGHT = 8
THICKNESS = 0.5
SIGNS = ((1, 1), (-1, 1), (-1, -1), (1, -1))  # round the square, the same on both


def square(corners):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y, z in corners:
        polygon.Add(gp_Pnt(x, y, z))
    polygon.Close()
    return polygon.Wire()


def frustum(top):
    """The loft up to z = `top`, each corner running straight on past z = 8."""
    c, s = math.cos(math.radians(30)), math.sin(math.radians(30))
    upper = [(2 * sx, 2 * sy) for sx, sy in SIGNS]
    lower = [(1 + c * 4 * sx - s * 4 * sy, s * 4 * sx + c * 4 * sy) for sx, sy in SIGNS]
    f = top / HEIGHT
    end = [(lx + (ux - lx) * f, ly + (uy - ly) * f, top)
           for (lx, ly), (ux, uy) in zip(lower, upper)]
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(square([(x, y, 0) for x, y in lower]))
    loft.AddWire(square(end))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the twisted frustum")
    return loft.Shape()


def build():
    # the cavity is the solid continued to z = 10, shrunk by 0.5 and cut off at z = 8
    return shell_to_plane(frustum(HEIGHT), frustum(10), THICKNESS, HEIGHT)
