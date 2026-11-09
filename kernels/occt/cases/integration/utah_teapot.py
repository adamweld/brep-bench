"""The Utah teapot. Martin Newell's teapot of 1975 as a solid: his 32 bicubic Bézier patches
(with the four of the bottom), in his own coordinates, z up, the teapot resting on the origin.
The patches bound no solid by themselves (the lid stands apart from the rim, the spout and the
handle run into the body with open ends, the spout's mouth is open), so the solid is the union
of four solids, each bounded by some of the patches and closed by flat faces:

- the body: the rim, the body and the bottom, closed by the disk of radius 1.4 in the plane
  z = 2.4 inside the rim;
- the lid: the knob and the lid, closed by the disk of radius 1.3 in the plane z = 2.4, which
  lies on the body's disk;
- the spout, closed at its base by the flat region in the plane x = 1.7 inside the base's loop,
  and at its mouth by the flat region in the plane z = 2.4 inside the lip's inner loop;
- the handle, closed by the flat region inside each end's loop, each loop lying in a plane.

The body, the lid and the bottom are rings of four patches, one a quarter turn round the z axis
from the next, each made from a profile of four points (r, z): the patch's row i is (r, 0, z),
(r, −0.56r, z), (0.56r, −r, z), (0, −r, z) for the profile's point i (0.5625 in place of 0.56 in
the knob's second row). The spout and the handle are halves mirrored in the plane y = 0. The
spout's base and the handle's upper end lie inside the body; the handle's lower end lies inside
it but for one point, (−2, 0, 0.9), where its loop touches the body's widest circle.

OCCT's volume of each part agrees with its exact value to 1e-14, but its adaptive integration of
area does not settle on these patches: a lip patch of the spout measures 0.14752, 0.14720 and
0.14699 asked to 1e-6, 1e-10 and 1e-12, and the parts' areas are off by up to 1e-4."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_MakeWire,
                                BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.Geom import Geom_BezierCurve, Geom_BezierSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array1_gp_Pnt, Array2_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused

# Profiles (r, z) of the rings, from the top of each down (or, for the lid, from the knob out).
RIM = [(1.4, 2.4), (1.3375, 2.53125), (1.4375, 2.53125), (1.5, 2.4)]
UPPER_BODY = [(1.5, 2.4), (1.75, 1.875), (2.0, 1.35), (2.0, 0.9)]
LOWER_BODY = [(2.0, 0.9), (2.0, 0.45), (1.5, 0.225), (1.5, 0.15)]
BOTTOM = [(1.5, 0.15), (1.5, 0.075), (1.425, 0.0), (0.0, 0.0)]
KNOB = [(0.0, 3.15), (0.8, 3.15), (0.0, 2.85), (0.2, 2.7)]
LID = [(0.2, 2.7), (0.4, 2.55), (1.3, 2.55), (1.3, 2.4)]

# The halves of the spout and the handle on the side y ≤ 0, each a 4 × 4 net of rows.
HANDLE = [
    [[(-1.6, 0.0, 2.025), (-1.6, -0.3, 2.025), (-1.5, -0.3, 2.25), (-1.5, 0.0, 2.25)],
     [(-2.3, 0.0, 2.025), (-2.3, -0.3, 2.025), (-2.5, -0.3, 2.25), (-2.5, 0.0, 2.25)],
     [(-2.7, 0.0, 2.025), (-2.7, -0.3, 2.025), (-3.0, -0.3, 2.25), (-3.0, 0.0, 2.25)],
     [(-2.7, 0.0, 1.8), (-2.7, -0.3, 1.8), (-3.0, -0.3, 1.8), (-3.0, 0.0, 1.8)]],
    [[(-2.7, 0.0, 1.8), (-2.7, -0.3, 1.8), (-3.0, -0.3, 1.8), (-3.0, 0.0, 1.8)],
     [(-2.7, 0.0, 1.575), (-2.7, -0.3, 1.575), (-3.0, -0.3, 1.35), (-3.0, 0.0, 1.35)],
     [(-2.5, 0.0, 1.125), (-2.5, -0.3, 1.125), (-2.65, -0.3, 0.9375), (-2.65, 0.0, 0.9375)],
     [(-2.0, 0.0, 0.9), (-2.0, -0.3, 0.9), (-1.9, -0.3, 0.6), (-1.9, 0.0, 0.6)]],
]
SPOUT = [
    [[(1.7, 0.0, 1.425), (1.7, -0.66, 1.425), (1.7, -0.66, 0.6), (1.7, 0.0, 0.6)],
     [(2.6, 0.0, 1.425), (2.6, -0.66, 1.425), (3.1, -0.66, 0.825), (3.1, 0.0, 0.825)],
     [(2.3, 0.0, 2.1), (2.3, -0.25, 2.1), (2.4, -0.25, 2.025), (2.4, 0.0, 2.025)],
     [(2.7, 0.0, 2.4), (2.7, -0.25, 2.4), (3.3, -0.25, 2.4), (3.3, 0.0, 2.4)]],
    [[(2.7, 0.0, 2.4), (2.7, -0.25, 2.4), (3.3, -0.25, 2.4), (3.3, 0.0, 2.4)],
     [(2.8, 0.0, 2.475), (2.8, -0.25, 2.475), (3.525, -0.25, 2.49375), (3.525, 0.0, 2.49375)],
     [(2.9, 0.0, 2.475), (2.9, -0.15, 2.475), (3.45, -0.15, 2.5125), (3.45, 0.0, 2.5125)],
     [(2.8, 0.0, 2.4), (2.8, -0.15, 2.4), (3.2, -0.15, 2.4), (3.2, 0.0, 2.4)]],
]


def quarter_turn(p, quarters):
    """`p` turned `quarters` quarter turns round the z axis, from +x toward −y."""
    x, y, z = p
    for _ in range(quarters):
        x, y = y, -x
    return (x, y, z)


def ring(profile, knob=False):
    """The four nets of a ring: row i from the profile's point i, then turned round the z axis."""
    net = []
    for i, (r, z) in enumerate(profile):
        k = 0.5625 if knob and i == 1 else 0.56
        net.append([(r, 0.0, z), (r, -k * r, z), (k * r, -r, z), (0.0, -r, z)])
    return [[[quarter_turn(p, q) for p in row] for row in net] for q in range(4)]


def mirrored(net):
    """A half's net mirrored in the plane y = 0 (each row reversed, so it runs on from the half)."""
    return [[(x, -y, z) for x, y, z in reversed(row)] for row in net]


def patch(net):
    poles = Array2_gp_Pnt(1, 4, 1, 4)
    for i, row in enumerate(net):
        for j, p in enumerate(row):
            poles.SetValue(i + 1, j + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeFace(Geom_BezierSurface(poles), 1e-9).Face()


def cap(rows):
    """The flat face inside the closed loop of cubic Bézier curves whose control points are `rows`."""
    wire = BRepBuilderAPI_MakeWire()
    for row in rows:
        poles = Array1_gp_Pnt(1, 4)
        for j, p in enumerate(row):
            poles.SetValue(j + 1, gp_Pnt(*p))
        wire.Add(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace found no plane through a cap's loop")
    return face.Face()


def solid(nets, caps, what):
    """The solid bounded by the patches `nets` and the flat faces inside the loops `caps`."""
    sewing = BRepBuilderAPI_Sewing(1e-7)
    for net in nets:
        sewing.Add(patch(net))
    for rows in caps:
        sewing.Add(cap(rows))
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused(f"BRepBuilderAPI_Sewing made no shell of the {what}'s faces")
    made = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not made.IsDone():
        raise Refused(f"BRepBuilderAPI_MakeSolid declined the {what}'s sewn shell")
    made = made.Solid()
    BRepLib.OrientClosedSolid_s(made)
    return made


def halves(nets):
    return nets + [mirrored(net) for net in nets]


def build():
    rim, upper, lower, bottom = ring(RIM), ring(UPPER_BODY), ring(LOWER_BODY), ring(BOTTOM)
    knob, lid = ring(KNOB, knob=True), ring(LID)
    body = solid(rim + upper + lower + bottom, [[net[0] for net in rim]], "body")
    lid = solid(knob + lid, [[net[3] for net in lid]], "lid")
    spout = solid(halves(SPOUT), [[SPOUT[0][0], mirrored(SPOUT[0])[0]],
                                  [SPOUT[1][3], mirrored(SPOUT[1])[3]]], "spout")
    handle = solid(halves(HANDLE), [[HANDLE[0][0], mirrored(HANDLE[0])[0]],
                                    [HANDLE[1][3], mirrored(HANDLE[1])[3]]], "handle")
    teapot = body
    for part, what in ((lid, "lid"), (spout, "spout"), (handle, "handle")):
        fuse = BRepAlgoAPI_Fuse(teapot, part)
        if not fuse.IsDone():
            raise Refused(f"BRepAlgoAPI_Fuse declined to join the {what} to the teapot")
        teapot = fuse.Shape()
    return teapot
