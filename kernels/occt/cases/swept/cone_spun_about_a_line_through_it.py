"""A cone spun about a line that crosses it aslant. The solid a cone fills as it is spun partway
about a line that crosses its axis aslant, inside it.

The cone's axis is the line through the origin along w = (cos 30°, 0, sin 30°), 60° from the z
axis. Its point is at −2w and it opens along +w at 50° (each side 25° from its axis) to a flat base
at +5w, of radius 7 tan 25°. The z axis meets its axis at the origin, inside it.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 150°. Make the solid of
everything the cone passes through.

The z axis runs through the cone, leaving it through its side above and below the origin. There the
turn holds the cone's side still, and the contact passes over those two points as well as down to
the cone's point."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from sweep import plane, turned, united

TILT = math.radians(30)  # of the cone's axis w, up from the x axis toward z
POINT, BASE = -2, 5  # along w, from the origin
HALF_ANGLE = math.radians(25)
TURN = 150  # degrees about the z axis


def build():
    w = (math.cos(TILT), 0, math.sin(TILT))
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(*(POINT * x for x in w)), gp_Dir(*w)), 0,
                                (BASE - POINT) * math.tan(HALF_ANGLE), BASE - POINT).Shape()
    # The cone's axis meets the turn's, at the origin o. At a point p = s w + ρ r̂ of its side
    # (r̂ square to w), its normal is n = cos 25°·r̂ − sin 25°·w, and the velocity is ẑ × p, so
    # n·(ẑ × p) = (s cos 25° + ρ sin 25°)·(ẑ × w)·r̂. That is zero where r̂ lies in the plane
    # y = 0 through both axes, and on the circle where s cos 25° + ρ sin 25° = 0, the circle
    # whose normals pass through o: with ρ = (s + 2) tan 25°, at s = −2 sin² 25°, which the
    # plane square to w there cuts out. The flat base meets the turn square-on along y = 0 too.
    # The lines in y = 0 cross the turn's axis, where it leaves the cone, and a curve turned
    # about an axis it crosses sweeps a surface that passes through itself, which MakeRevol
    # declines: the plane x = 0 through the axis cuts the cone again there.
    circle = -2 * math.sin(HALF_ANGLE) ** 2
    cuts = [plane((0, 0, 0), (0, 1, 0)), plane(tuple(circle * x for x in w), w),
            plane((0, 0, 0), (1, 0, 0))]
    return united(turned(cone, gp.OZ_s(), TURN, cuts))
