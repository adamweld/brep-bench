//! #3DBenchy, as near as a few solids come to it, from numbers read off the reference (the
//! numbers cadrs's and cad-kernel's likenesses read there, `kernels/cadrs`): a likeness, not the
//! original's 1385 faces.
//!
//! The hull's side is the reference's own surface, its 12 × 6 bicubic net written out, and its
//! mirror image, sewn with a ruled stern between their ends and the flat faces their rims bound
//! at z = 0 and z = 30 into a solid (the faces made by the kernel on the surfaces given as data,
//! `shared::nurbs`, and sewn by ops), kept below the sheer (an arc of radius 103.08 aft, the
//! gunwale's plane at the bow). The deck well is a ruled loft through its plan at four heights (a
//! bilinear sheet for each side of each band, sewn with the lowest and highest plans), kept above
//! the deck; ops's offset of the hull's sides (cadrs's well) is refused, "how far its neighbours
//! extend to meet". The wheelhouse, its room, the roof and the cargo box are
//! SolidCraft's convex polyhedra of the reference's own planes (the walls leaning 1.5°, 4.5° and
//! 3.2°, the roof 5.5°, the box 5°); the doors, the windows, their frames, the wheel, the chimney,
//! the rod holder, the drain and the hawse pipes are cylinders, a cone and rounded rectangles on
//! the reference's own axes. Everything is taken to ops through its record and joined there.
use crate::bench::{arc, common, cut, cut_all, fuse, fuse_all, line, polygon};
use crate::shared::nurbs::{polygon as flat, segment, sewn, spline, spline_curve, Face};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, Curve, DraftCorner};
use solidcore_ops::{make_box, make_cone, Body};
use solidcore_record::Vec3;
use solidcraft_kernel::{brep_record, convex_polyhedron, HalfSpace};
use std::f64::consts::FRAC_PI_2;

type V = [f64; 3];

/// The hull's side: its net's rows from the stern to the stem, each the x and y of a pole at each
/// of the heights `Z`.
const Z: [f64; 6] = [0., 3.31262547, 9.86113758, 19.99948838, 26.66940917, 30.];
const NET: [[f64; 12]; 12] = [
    [-25., 6.5, -26.57777641, 8.19743825, -29.69677916, 11.55299264, -30.07483416, 13.10684206, -30.02492292, 13.70254086, -30., 14.],
    [-16.66666667, 7., -18.03251666, 8.91666531, -20.73257597, 12.7055953, -19.33270927, 13.98651841, -19.33312549, 14.21782938, -19.33333333, 14.33333333],
    [-6.95026665, 7.582984, -9.02800892, 9.6451771, -13.13536141, 13.72179144, -9.0559802, 14.83019903, -9.10125023, 14.71160089, -9.12385558, 14.65237951],
    [1.70173289, 8.10210397, 1.0378164, 10.52894472, -0.27463657, 15.32640711, 4.8956376, 15.97006815, 4.78090388, 15.37984019, 4.72361218, 15.08511288],
    [3.27390994, 7.90566507, 3.00533353, 10.39680401, 2.47440245, 15.32137327, 7.73446515, 15.80894762, 7.58959184, 15.1603348, 7.51725009, 14.8364533],
    [6.12303775, 7.14157538, 6.77506979, 9.59134478, 8.06402917, 14.43413329, 13.32229534, 14.65025381, 13.12364577, 13.96396373, 13.02445113, 13.62126829],
    [7.40374132, 6.57336961, 8.57825828, 8.91726822, 10.90008385, 13.55076773, 16.06944082, 13.6522451, 15.84751934, 12.98681943, 15.73670399, 12.65454253],
    [9.71034637, 5.16566225, 11.90317743, 7.0926504, 16.23804139, 10.90198694, 21.17885739, 10.87503743, 20.95982379, 10.32029188, 20.85045048, 10.04328252],
    [10.73847272, 4.32482338, 13.42488446, 5.94169581, 18.7354757, 9.13798492, 23.53919073, 9.09648792, 23.34674437, 8.6314171, 23.25064727, 8.39918636],
    [12.55886666, 2.37589024, 16.07946535, 3.24321782, 23.03910609, 4.95778086, 27.62607126, 4.93455077, 27.56963652, 4.68590901, 27.54145612, 4.56175102],
    [13.3525947, 1.26641564, 17.21234735, 1.69588126, 24.84243935, 2.5448637, 29.35130573, 2.55271285, 29.4045503, 2.43042649, 29.4311377, 2.36936342],
    [14., 0., 18.10315969, 0., 26.21442708, 0., 30.6839943, 0., 30.89475682, 0., 31., 0.],
];
const UKNOTS: [f64; 16] = [
    0., 0., 0., 0., 0.505209458052, 0.505209458052, 0.628907093539, 0.628907093539, 0.752604729026,
    0.752604729026, 0.876302364513, 0.876302364513, 1., 1., 1., 1.,
];
const VKNOTS: [f64; 10] = [0., 0., 0., 0., 0.33885526461, 0.669860758304, 1., 1., 1., 1.];
/// The deck well's plan at each height, its half on y ≥ 0 from the stem round to the stern (read
/// on thirteen rays from the origin, as cad-kernel's likeness reads it).
const BOW: [[f64; 2]; 4] = [[27.77, 0.], [26.11, 3.67], [21.05, 7.66], [13.93, 11.69]];
const WELL: [(f64, [[f64; 2]; 9]); 4] = [
    (8.6, [[7.41, 10.59], [3.97, 10.91], [0.97, 11.08], [-1.92, 10.89], [-8.54, 10.18],
           [-17.78, 10.26], [-23.57, 8.58], [-26.17, 4.62], [-27.04, 0.]]),
    (10.5, [[7.41, 10.59], [4.37, 12.02], [1.07, 12.2], [-2.11, 11.97], [-9.38, 11.18],
            [-17.78, 10.26], [-25.8, 9.39], [-27.06, 4.77], [-27.6, 0.]]),
    (12., [[8.22, 11.74], [4.57, 12.56], [1.11, 12.72], [-2.2, 12.45], [-9.38, 11.18],
           [-17.78, 10.26], [-25.8, 9.39], [-27.06, 4.77], [-28.04, 0.]]),
    (28., [[9.54, 13.63], [4.57, 12.56], [1.11, 12.72], [-2.2, 12.45], [-9.38, 11.18],
           [-17.78, 10.26], [-25.8, 9.39], [-27.06, 4.77], [-28.04, 0.]]),
];
/// The deck (x, z), and the well above it.
const DECK: [[f64; 2]; 6] =
    [[-40., 8.5], [5.95, 8.5], [10.12, 14.46], [40., 23.42], [40., 60.], [-40., 60.]];
/// The aft sheer's circle in (x, z): its centre and its radius; the transom.
const SHEER: [f64; 3] = [-23.0812, 113.7248, 103.0821];
const STERN: f64 = -29.1765;
/// How deep the rail is under the sheer, its upright outer side's circle in (x, y) (mirrored for
/// the other side) and where it ends forward.
const RAIL: f64 = 1.7;
const UPRIGHT: [f64; 3] = [-57.949424, 330.686828, 320.92693];
const AFT_RAIL: f64 = -1.009;
/// The sines and cosines of the walls' leans: the wheelhouse's back 1.5°, its front 4.5°, the
/// roof 5.5° and the cargo box 5°.
const S15: f64 = 0.02618;
const C15: f64 = 0.99966;
const S45: f64 = 0.07846;
const C45: f64 = 0.99692;
const S55: f64 = 0.09585;
const C55: f64 = 0.99540;
const S5: f64 = 0.08716;
const C5: f64 = 0.99619;

fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn scale(a: V, s: f64) -> V {
    a.map(|c| c * s)
}

fn cross(a: V, b: V) -> V {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn unit(a: V) -> V {
    scale(a, 1. / (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt())
}

fn vec3([x, y, z]: V) -> Vec3 {
    Vec3::new(x, y, z)
}

/// A body of SolidCraft's kernel taken to ops through its record.
fn ops(body: solidcraft_kernel::Body) -> Result<Body, Refusal> {
    Ok(Body::from_record(&brep_record(&body)?, true)?.0)
}

/// The common part of the half-spaces a·x + b·y + c·z ≤ d and the box |x|, |y|, |z| ≤ 60:
/// SolidCraft's convex polyhedron of those planes.
fn convex(planes: &[[f64; 4]]) -> Result<Body, Refusal> {
    let bounds = [[1., 0., 0.], [-1., 0., 0.], [0., 1., 0.], [0., -1., 0.], [0., 0., 1.], [0., 0., -1.]];
    let all = planes.iter().copied().chain(bounds.iter().map(|&[a, b, c]| [a, b, c, 60.]));
    let half = all.map(|[a, b, c, d]| {
        let k = (a * a + b * b + c * c).sqrt();
        HalfSpace { n: Vec3::new(a / k, b / k, c / k), d: d / k }
    });
    ops(convex_polyhedron(&half.collect::<Vec<_>>())?)
}

fn cylinder(foot: V, along: V, r: f64, h: f64) -> Result<Body, Refusal> {
    ops(solidcraft_kernel::cylinder(vec3(foot), vec3(unit(along)), r, h)?)
}

/// The closed loop of curves `profile` extruded `depth` along the unit `along`.
fn prism(profile: &[Curve], along: V, depth: f64) -> Result<Body, Refusal> {
    Ok(extrude(profile, along, depth, 0., DraftCorner::Curvature, true)?)
}

/// The prism through y = ±20 of an outline in the xz plane: corners joined by lines, but where a
/// circle's centre and radius follow a corner, by the short arc of that circle to the next,
/// counter-clockwise looking along −y (from x toward z).
fn across(corners: &[([f64; 2], Option<[f64; 3]>)]) -> Result<Body, Refusal> {
    let at = |[x, z]: [f64; 2]| [x, -20., z];
    let n = corners.len();
    let edges: Vec<Curve> = (0..n)
        .map(|k| {
            let (a, b) = (corners[k].0, corners[(k + 1) % n].0);
            match corners[k].1 {
                None => line(at(a), at(b)),
                Some([cx, cz, r]) => {
                    let angle = |[x, z]: [f64; 2]| (z - cz).atan2(x - cx);
                    arc([cx, -20., cz], [0., -1., 0.], [1., 0., 0.], r, angle(a), angle(b))
                }
            }
        })
        .collect();
    prism(&edges, [0., 1., 0.], 40.)
}

/// A rectangle [u0, u1] × [v0, v1] with its corners rounded 0.5, on the plane through `origin`
/// with sketch x along `x` and normal `normal` (sketch y their cross product, `normal` × `x`),
/// extruded `depth` along the normal.
fn rounded(origin: V, x: V, normal: V, [u0, u1, v0, v1]: [f64; 4], depth: f64) -> Result<Body, Refusal> {
    let r = 0.5;
    let y = cross(normal, x);
    let at = |u: f64, v: f64| add(origin, add(scale(x, u), scale(y, v)));
    // each corner's quarter, counter-clockwise about the normal from the angle `from`
    let corner = |u: f64, v: f64, from: f64| arc(at(u, v), normal, x, r, from, from + FRAC_PI_2);
    let q = FRAC_PI_2;
    let edges = vec![
        line(at(u0 + r, v0), at(u1 - r, v0)),
        corner(u1 - r, v0 + r, -q),
        line(at(u1, v0 + r), at(u1, v1 - r)),
        corner(u1 - r, v1 - r, 0.),
        line(at(u1 - r, v1), at(u0 + r, v1)),
        corner(u0 + r, v1 - r, q),
        line(at(u0, v1 - r), at(u0, v0 + r)),
        corner(u0 + r, v0 + r, 2. * q),
    ];
    prism(&edges, normal, depth)
}

/// The side net (its mirror image for `s` = −1) as poles.
fn side(s: f64) -> Vec<Vec<V>> {
    NET.iter().map(|row| (0..6).map(|j| [row[2 * j], s * row[2 * j + 1], Z[j]]).collect()).collect()
}

/// The side net and its mirror image, sewn with the stern and the flat faces at z = 0 and z = 30.
fn hull() -> Result<Body, Refusal> {
    let (port, starboard) = (side(1.), side(-1.));
    let stern = vec![port[0].clone(), starboard[0].clone()];
    let mut faces = vec![
        Face::Surface(spline(&UKNOTS, &VKNOTS, port.clone())),
        Face::Surface(spline(&UKNOTS, &VKNOTS, starboard.clone())),
        Face::Surface(spline(&[0., 0., 1., 1.], &VKNOTS, stern)),
    ];
    for j in [0, 5] {
        let rim = |poles: &Vec<Vec<V>>| spline_curve(&UKNOTS, &poles.iter().map(|r| r[j]).collect::<Vec<_>>());
        faces.push(Face::Flat(vec![rim(&port), segment(port[0][j], starboard[0][j]), rim(&starboard)]));
    }
    sewn(&faces)
}

/// The deck well's plans lofted: a bilinear sheet for each side of each band between two plans,
/// sewn with the lowest and the highest.
fn well() -> Result<Body, Refusal> {
    let rings: Vec<Vec<V>> = WELL
        .iter()
        .map(|(z, aft)| {
            let half: Vec<_> = BOW.iter().chain(aft.iter()).collect();
            // counter-clockwise about z: up the port side from the stem, back down the starboard
            half[..12].iter().map(|&&[x, y]| [x, y, *z])
                .chain(half[1..].iter().rev().map(|&&[x, y]| [x, -y, *z]))
                .collect()
        })
        .collect();
    let mut faces = vec![Face::Flat(flat(&rings[0])), Face::Flat(flat(&rings[rings.len() - 1]))];
    for w in rings.windows(2) {
        let n = w[0].len();
        for k in 0..n {
            let poles = vec![vec![w[0][k], w[1][k]], vec![w[0][(k + 1) % n], w[1][(k + 1) % n]]];
            faces.push(Face::Surface(spline(&[0., 0., 1., 1.], &[0., 0., 1., 1.], poles)));
        }
    }
    sewn(&faces)
}

pub fn build() -> Result<Body, Refusal> {
    let [cx, cz, r] = SHEER;
    let sheer = |x: f64| [x, cz - (r * r - (x - cx) * (x - cx)).sqrt()];
    // round from the transom's top: the sheer's arc forward, the gunwale's plane at the bow, the
    // stem and the keel's level
    let side = across(&[
        (sheer(STERN), Some(SHEER)),
        (sheer(4.16), None),
        ([10.22, 21.36], None),
        ([32., 27.89], None),
        ([32., -1.], None),
        ([STERN, -1.], None),
    ])?;
    let deck = prism(&polygon(&DECK.map(|[x, z]| [x, -20., z])), [0., 1., 0.], 40.)?;
    let hull = hull()?;
    let well = common(well()?, deck)?;
    let boat = cut(common(hull, side)?, well)?;
    // the rail aft of the wheelhouse: 1.7 deep under the sheer, its outer side upright (a
    // cylinder about z on each side) where the hull flares out below
    let mut slivers = Vec::new();
    for s in [1., -1.] {
        let rail = cylinder([cx, -20., cz], [0., 1., 0.], r + RAIL, 40.)?;
        let wall = cylinder([UPRIGHT[0], s * UPRIGHT[1], -10.], [0., 0., 1.], UPRIGHT[2], 60.)?;
        let aft = make_box([-40., s.min(0.) * 20., -10.], [AFT_RAIL + 40., 20., 60.])?;
        slivers.push(common(common(aft, rail)?, wall)?);
    }
    let boat = cut_all(boat, slivers)?;
    // the wheelhouse: its walls, less its room and the room's upper part over the console
    let walls = [[C15, 0., S15, -4.7751], [0.05545, -0.99812, 0.02618, -5.1525],
                 [0.05545, 0.99812, 0.02618, -5.1525], [0., 0., -1., -36.614]];
    let room = |front: f64, floor: f64| {
        let mut planes: Vec<_> = walls.iter().map(|p| p.map(|c| -c)).collect();
        planes.extend([[C45, 0., -S45, front], [0., 0., -1., -floor]]);
        convex(&planes)
    };
    let house = convex(&[[-C15, 0., -S15, 6.7751], [C45, 0., -S45, 10.2992], [0., 0., -1., -8.],
                         [-0.05545, 0.99812, -0.02618, 7.1525], [-0.05545, -0.99812, -0.02618, 7.1525],
                         [0., 0., 1., 38.]])?;
    let rooms = [room(5.8069, 8.5)?, room(8.2992, 20.5)?];
    let roof = convex(&[[-S55, 0., C55, 38.7973], [C55, 0., S55, 17.7101], [-C55, 0., -S55, 5.2885],
                        [-0.05745, 0.99833, -0.00553, 8.8967], [-0.05745, -0.99833, -0.00553, 8.8967],
                        [S55, 0., -C55, -35.7973]])?;
    let cargo = convex(&[[-C5, 0., -S5, 16.7295], [0., C5, -S5, 4.7752], [0., -C5, -S5, 4.7752],
                         [0., 0., 1., 15.5], [0., 0., -1., -8.], [1., 0., 0., -6.],
                         [-1., 0., 0., 17.9984], [0., 1., 0., 5.9984], [0., -1., 0., 5.9984]])?;
    let rod = common(cylinder([-29.939, 0., 22.073], [0.28046, 0., -0.95987], 1.75, 20.)?,
                     convex(&[[0., 0., 1., 12.87], [0., 0., -1., -8.]])?)?;
    let up = [0., 0., 1.];
    // the chimney: a cone widening 2° upward from r 2.75 at z 37, under a lip from z 45 to 48
    let flare = 2f64.to_radians().tan();
    let chimney = make_cone([-4., 0., 36.], 2.75 - flare, 2.75 + 8. * flare, 9.)?;
    let lip = cylinder([-4., 0., 45.], up, 3.5, 3.)?;
    // the back window's frame: r 6, 0.3 proud of the wall
    let back = [C15, 0., S15];
    let ring = cylinder(add([-0.718, 0., 27.419], scale(back, -7.0751)), back, 6., 0.5)?;
    let mut stack = vec![house, roof, cargo, rod, chimney, lip, ring];
    for s in [1., -1.] {
        // a door's frame: 1.5 round the doorway, 0.3 proud of the wall
        let slab = [[-0.05545, 0.99812 * s, -0.02618, 7.4525], [0.05545, -0.99812 * s, 0.02618, -6.9]];
        let mut jambs = vec![[-0.99812, -0.0561 * s, -0.02475, 2.8166], [0.99538, 0.0532 * s, -0.07997, 6.2812],
                             [0., 0., -1., -8.5], [0., 0., 1., 27.52]];
        jambs.extend(slab);
        let arch = cylinder([2.392 - 0.05545 * s * 4., 2.0057 * s + 4. * s, 27.5982 - 0.02618 * 4.],
                            [-0.05545, s, -0.02618], 6., 8.)?;
        let above = convex(&[slab[0], slab[1], [0., 0., -1., -27.52]])?;
        stack.push(fuse(convex(&jambs)?, common(arch, above)?)?);
    }
    // the front window's frame: 1.5 round the window, 0.3 proud of the wall, square to it
    let front = [C45, 0., -S45];
    stack.push(rounded(scale(front, 10.), [0., 1., 0.], front, [-6.75, 6.75, 24.0213, 36.5512], 0.5992)?);
    let boat = fuse_all(std::iter::once(boat).chain(stack))?;
    // (the rooms after the roof: the house's ceiling is flat under its sloping roof)
    let boat = cut_all(boat, rooms)?;
    // the wheel on the console, square to the front wall: a disk 1.5 thick, its face sunk 0.5
    // between its rim and its hub
    let face = add([1.581, 0., 20.086], scale(front, 5.0569));
    let wheel = cut(cylinder(face, front, 5., 1.5)?, cylinder(add(face, scale(front, -1.)), front, 3.5, 1.5)?)?;
    let boat = fuse_all([boat, wheel, cylinder(face, front, 1.25, 0.5)?])?;
    // the cargo box's hold, the chimney's bore, the front and rear windows, the rod holder's bore
    // and the drain
    let mut cuts = vec![
        make_box([-15.998, -3.998, 6.5], [7.003, 7.996, 9.5])?,
        cylinder([-4., 0., 37.], up, 1.5, 12.)?,
        rounded([9., 0., 0.], [0., 1., 0.], [1., 0., 0.], [-5.25, 5.25, 24.611, 34.112], 7.)?,
        cylinder([-10., 0., 27.1755], back, 4.5, 6.)?,
        cylinder([-24.8065, 0., 4.508], [-0.28046, 0., 0.95987], 1., 10.)?,
        cylinder([-26.5, 0., 1.6], [1., 0., 0.], 1., 6.5)?,
    ];
    for s in [1., -1.] {
        // a doorway, the door below its arch and the arch, and a hawse pipe (the doorway one
        // cutter: cut one after the other, they leave a body the kernel makes no record of, "a
        // conic's NURBS is off its own frame and radii")
        let door = convex(&[[-0.99812, -0.0561 * s, -0.02475, 1.3166], [0.99538, 0.0532 * s, -0.07997, 4.7812],
                            [0., 0., -1., -8.5], [0., 0., 1., 27.52], [0., -s, 0., -2.], [0., s, 0., 9.5]])?;
        cuts.push(fuse(door, cylinder([2.392, 2.0057 * s, 27.5982], [-0.05545, s, -0.02618], 4.5, 8.)?)?);
        cuts.push(cylinder([20.6648, 1.0843 * s, 21.4209], [0.70711, 0.6984 * s, -0.11062], 2., 12.)?);
    }
    cut_all(boat, cuts)
}
