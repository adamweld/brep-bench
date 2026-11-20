r"""A smooth loft through a triangle, a square, an octagon, a square and a triangle, between two points. A smooth loft through five stacked polygons of different numbers of sides, capped at each end by a point: a tower that swells from a triangle to an octagon and back, its two ends closing to tips.

The sections are regular polygons centered on the z axis, in horizontal planes:

| z | polygon | circumradius | a corner at azimuth |
|---|---|---|---|
| 0 | triangle | 2.5 | 90° |
| 2 | square | 3.5 | 45° |
| 4 | octagon | 4 | 0° |
| 6 | square | 3.5 | 45° |
| 8 | triangle | 2.5 | 270° |

The end points are $P_0 = (0.6, 0.4, -2)$, 2 below the first polygon, and $P_6 = (-0.4, -0.6, 10)$, 2 above the last. Together with the polygons they make seven sections, $k = 0 \dots 6$, at the heights z = −2, 0, 2, …, 10.

Each polygon is taken as a closed polyline of 24 stations, $S_j$ for $j = 0 \dots 23$, the station $S_j$ being the point of its boundary at azimuth 15°·j about the z axis (a ray from the axis at that azimuth meets the boundary once); every corner of every polygon is one of its stations. For each station j there is a curve $Q_j(t)$, $t \in [0, 1]$, through the seven points $S_j^k$, k = 0 … 6, taken at $t = k/6$ ($S_j^0 = P_0$ and $S_j^6 = P_6$ for every j, and $S_j^k$ the station j of the polygon k for k = 1 … 5): the natural cubic spline, each of its three coordinates a piecewise cubic with two continuous derivatives that has zero second derivative at t = 0 and at t = 1. Its height is z = −2 + 12t exactly. The loft is the surface of the points $(1 - f)\,Q_j(t) + f\,Q_{j+1}(t)$ for j = 0 … 23 (j + 1 taken round), f in [0, 1], t in [0, 1]: at each t the polyline through the points $Q_j(t)$, filled straight between stations. At t = 0 and t = 1 the polyline has shrunk to the point $P_0$ and the point $P_6$, so the solid is closed by this surface alone: it has no flat faces.

Finding: OCCT's smooth BRepOffsetAPI_ThruSections through the same vertices and wires is not this
law: it gives volume 226.63 and area 245.57, against 220.56 and 209.66 for the stated surface (the
integral of its law). So the solid is built here exactly: each strip is a Geom_BSplineSurface of
degree 1 in f and 3 in t, the poles of the two natural-spline curves its rows (computed with
scipy, for the stated law's B-spline form), the 24 strips sewn."""

import math

import numpy as np
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeSolid, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_Sewing)
from OCP.Geom import Geom_BSplineSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt
from OCP.collections import Array1_double, Array1_int, Array2_gp_Pnt
from scipy.interpolate import make_interp_spline

from bench import Refused

SECTIONS = [(3, 2.5, 90, 0), (4, 3.5, 45, 2), (8, 4.0, 0, 4), (4, 3.5, 45, 6), (3, 2.5, 270, 8)]
LOWER, UPPER = (0.6, 0.4, -2), (-0.4, -0.6, 10)


def station(n, circumradius, corner_deg, j):
    """The point of the regular n-gon's boundary at azimuth 15 j degrees."""
    step = 2 * math.pi / n
    apothem = circumradius * math.cos(math.pi / n)
    phi = math.radians(15 * j)
    local = (phi - math.radians(corner_deg)) % step
    r = apothem / math.cos(local - step / 2)
    return (r * math.cos(phi), r * math.sin(phi))


def curve_poles(j):
    """The B-spline poles (degree 3, knots at k/6, ends fourfold) of the natural cubic spline
    through the seven points of station j at t = k/6."""
    pts = [LOWER]
    for n, radius, corner, z in SECTIONS:
        x, y = station(n, radius, corner, j)
        pts.append((x, y, z))
    pts.append(UPPER)
    spline = make_interp_spline(np.arange(7) / 6, np.array(pts), k=3, bc_type="natural")
    return spline.t, spline.c


def build():
    # Each strip between stations j and j+1 is the surface (1 − f) Q_j(t) + f Q_j+1(t): a
    # B-spline surface of degree 1 in f and degree 3 in t, the poles of the two curves its two
    # rows. The strips are sewn into a shell; the two tips are their degenerate edges.
    curves = [curve_poles(j) for j in range(24)]
    knots_t = curves[0][0]
    distinct = sorted(set(np.round(knots_t, 12)))
    mult = [int(np.sum(np.isclose(knots_t, k))) for k in distinct]
    uk, um = Array1_double(1, 2), Array1_int(1, 2)
    uk.SetValue(1, 0.0); uk.SetValue(2, 1.0); um.SetValue(1, 2); um.SetValue(2, 2)
    vk, vm = Array1_double(1, len(distinct)), Array1_int(1, len(distinct))
    for i, (k, m) in enumerate(zip(distinct, mult), 1):
        vk.SetValue(i, float(k)); vm.SetValue(i, m)
    sewing = BRepBuilderAPI_Sewing(1e-6)
    for j in range(24):
        poles = Array2_gp_Pnt(1, 2, 1, len(curves[j][1]))
        for row, c in enumerate((curves[j][1], curves[(j + 1) % 24][1]), 1):
            for col, (x, y, z) in enumerate(c, 1):
                poles.SetValue(row, col, gp_Pnt(float(x), float(y), float(z)))
        surface = Geom_BSplineSurface(poles, uk, vk, um, vm, 1, 3)
        sewing.Add(BRepBuilderAPI_MakeFace(surface, 1e-7).Face())
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of the 24 strips")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    return solid.Shape()
