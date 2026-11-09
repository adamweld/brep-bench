# Tumbling-torus regressions

Ten cases isolate geometric triggers from seven failing Solvent renders. They are continuous
solids defined in ordinary geometric language, not meshes captured from a failing run. All are
unsettled: no measure from one kernel is used as an expected answer.

The two pipelines must be distinguished. The original failures are in **Solvent's approximate
material-field preview**, after a successful constraint solve. brep-bench calls **cad-kernel's
native B-rep ring sweep**, which can refuse earlier while constructing the envelope. A native
refusal below does not reproduce the field mesher's diagnostic, and an operand refusal does not
exercise the Boolean. Keeping the separate operands makes that distinction visible on the board.

## Versions and evidence

- Solvent/geomsolver: `55e64186dfd751b5c9f269e8889e93b38fdb06d4`, source build, default preview
  fineness unless stated otherwise. All original documents solve to DOF 0.
- brep-bench base: `6afc44e8328ce3be4d0c6c1430a8bf746a166635`.
- Native adapter: the unchanged, pinned **cad-kernel 0.6.2** release from crates.io.
- OCCT adapter: refuses the same unavailable continuous rigid-motion sweep as the existing
  `swept/tumbling-torus` implementation. No finite union of sampled poses is substituted.

The descriptions under `cases/` define the benchmark solids. The investigation used Solvent
models with lengths in millimeters; benchmark lengths are unitless with the same numeric values.
Solvent's front-plane coordinates (u, v) map to (x, 0, z).

## Corrected axis label

The original case 07 was mislabeled “nearly parallel.” Its source actually sets the observer
axis to `(cos 89°, sin 89° cos 1°, sin 89° sin 1°)`, about **89.00015° from z**. The two
axes are almost perpendicular, not parallel, and their rotations do not nearly cancel. The
`near-orthogonal` cases preserve the actual failing geometry and correct the name; no axis
was silently changed to match the old label.

## What changes the failures

Each comparison holds the other parameters fixed. “Renders” means a nonempty field preview was
produced; it is not proof of an exact B-rep, fabrication tolerance, volume, or area.

| Original input | Changed parameter or selected solid | Observed result |
| --- | --- | --- |
| needle | tube radius 0.015, 0.03, 0.06 | no boundary |
| needle | tube radius 0.12 or 0.25 | renders |
| needle | either turn's ratio set to zero | still no boundary |
| baseline tumble | tube radius reduced from 0.5 to 0.015 | no boundary |
| near-orthogonal | tube radius 0.08 | no boundary |
| near-orthogonal | tube radius 0.16 or 0.32 | renders |
| near-orthogonal | heading changed from 89° to 0° | renders |
| near-orthogonal | fineness 4, same geometry | finishes with 74,164 triangles |
| multi-turn | roll −45°…45° or −90°…90° | renders |
| multi-turn | roll −180°…180°, −360°…360°, or −540°…540° | non-manifold repair fails |
| crossed pair | either swept operand alone | renders |
| crossed pair | union or first minus second | feature protection fails |
| crossed pair | intersection | non-manifold repair fails |
| original thin crossed difference (tube 0.045) | either operand alone | already has no boundary |
| original thin crossed difference | tube 0.12 or 0.22 | reaches non-manifold repair failure |

The last original case is not a distinct Boolean regression: its operands are already missing.
It is covered by thin-material cases and the independently checked crossed operands rather than
added as an eleventh near-duplicate. The multi-turn case reduces from a 1080° interval to 360°
without losing the failure; the 180° interval is retained as its control.

## Failure mechanisms, and what remains unproved

### Thin material: cached signs and inadequate initial coverage

For the original needle input, the point **(4, 0, 2)** is the center of the torus tube at roll 0,
which lies inside the sweep interval. It is strictly interior. A public `MaterialField::query`
using `Source::Exact` gives **−0.015**, but the final preview's cached resolution gives
**+0.00243246117594**. This is an observed sign reversal, not an inference from an empty PNG.

The conservative support diagonal is 42.02099014. At fineness 1 the final cache's finest cell is
0.04202099, greater than the tube's diameter 0.03; its coarsest cell is 1.05052475. These come from
`FieldMesher`'s support-scaled criteria. At that witness, cached readings at fineness 2 and 4
become negative, but the full mesher still finishes empty even at fineness 4. Fixing one sign is
therefore insufficient: initial surface coverage is also implicated. At fineness 1 the first pass
inserts no points and returns no triangles; the final pass inserts only six and returns no
triangles. Both passes then have no feature curves to protect.

The simplified thin benchmark keeps the existing baseline's motion and placement, changing only
its tube radius to 0.015. It reproduces the empty-boundary outcome even with the baseline's exact
observer point (0, 0.5, 0.5). The exact baseline at tube radius 0.5 is the existing control.

### Nearly perpendicular axes: a thin boundary is found and then lost

This input is not another case where every seed misses. At fineness 1 its first pass briefly has
20 triangles and its final pass 58; each reaches repair with zero triangles. At the known
interior point (3.02, 0, 1), the exact and cached readings are both negative. At fineness 4 the
same geometry produces a final non-provisional surface of 37,082 vertices and 74,164 triangles.
Doubling only the tube radius also restores a preview.

This isolates sensitivity to thin, nearly equal-rate motion and resolution. The precise
refinement event that removes the last boundary, and full geometric accuracy of the finer
result, have not been established. Do not describe this input as an empty mathematical solid.

### Overlap and intersection: manifold repair runs out of actionable protection

The original multi-turn input reaches the final pass with 15 traced feature curves. It undergoes
five rebuilds before refusing a non-manifold boundary. A shorter, still failing full-turn
interval ends with 57 reported faults and **zero blocking balls**, after earlier rebuilds had
one blocking ball. The error does not necessarily mean that every ball is at its smallest size:
there may be no ball identified for shrinking.

The crossed intersection shows another path to that same message. Its first pass is empty,
so it traces **zero** feature curves. Its final pass finds thousands of triangles but fails
manifold repair with four faults and **no protecting balls at all**. Each operand renders by
itself. This points to the coarse pass missing the intersection's features before final
refinement tries to close their sharp junctions. It does not prove that the mathematical
intersection is non-manifold.

The standalone document matters for reproduction: when all three Boolean roots were placed in
one document and rendered in one process, the intersection produced a preview, while the union
and difference still failed protection. That run does not establish which changed detail
(solve perturbation, field/cache state, or traversal) tipped the result. It is additional evidence
of numerical sensitivity, not proof of a root cause. The observations above were made with
standalone documents, one Boolean result per run.

### Union and difference: close feature curves hit the spacing floor

The union traces 21 feature curves, then refuses during protection, before final surface
refinement. Its reported colliding ball centers are 0.00143206925 apart, with radii summing to
approximately 0.0015887. The difference has another collision: distance 0.00102649624, radii
summing to approximately 0.0017255. These are balls belonging to different traced curves.

`protect.rs` sets the target spacing for separated curves to `max(0.9*d/1.4, edge_size/1024)`.
For this input `edge_size/1024` is approximately **0.00126290267**. The unfloored targets are
0.00092061595 (union) and 0.00065989044 (difference), both below it. The reported gaps are already
below the allowed target, so the next protection round has no further spacing change to make,
even though the balls overlap. This establishes the immediate stopping condition. Whether those
curves need smaller protection, splitting, joining, or deduplication needs further investigation;
no assertion that they are duplicate geometry is made here.

Relevant geomsolver source at the pinned revision:

- `rust/gcs-solve/src/solid/field_mesh.rs`: support-scaled criteria; coarse pass → crease tracing
  → protected final pass.
- `rust/gcs-solve/src/solid/field/adf.rs`: cached interpolation and subdivision.
- `rust/gcs-solve/src/delaunay/refine/seed.rs`: sign-change rays and fallback lattice.
- `rust/gcs-solve/src/delaunay/refine/protect.rs`: spacing floor and overlap refusal.
- `rust/gcs-solve/src/delaunay/refine/progressive.rs`: manifold repair and ball shrink/rebuild.
- `rust/gcs-solve/src/solid/evaluated.rs`: empty vertices/facets become “no boundary yet”.

## Benchmark mapping and native results

Every new case has a cad-kernel implementation using public `build::torus`, `Trajectory::Relative`
and `swept::closed::ring`. The three Booleans use `boolean::combined` on the two independently
built sweeps. Membership-mesh sag and fit tolerance scale with the tube radius, as in the baseline;
no geometry, envelope, or expected measure is computed outside the kernel.

The controls below control the **field failure**, not a promised native success.

| Case suffix after `swept/tumbling-torus-` | Purpose | cad-kernel 0.6.2 |
| --- | --- | --- |
| `thin` | baseline motion, thin tube only | `Folded` while fitting the sheet |
| `near-orthogonal` | thin region under nearly equal-rate turns | two sheet crossings end at one rim point |
| `near-orthogonal-thick` | same motion, twice the tube radius | same rim-crossing refusal |
| `full-turn` | reduced interval retaining overlap failure | `Folded` while fitting the sheet |
| `half-turn` | only the interval shortened | B-spline pieces do not close |
| `crossed-forward` | first operand in isolation | `Curved` while constructing the sweep |
| `crossed-reverse` | second operand in isolation | `Curved` while constructing the sweep |
| `crossed-union` | feature-curve interaction | forward operand refuses; Boolean not reached |
| `crossed-intersection` | sharp junction / manifold closure | forward operand refuses; Boolean not reached |
| `crossed-difference` | close surviving feature curves | forward operand refuses; Boolean not reached |

Native `rings.rs` currently requires contact curves' piece kinds to match over the roll and
rejects changes as `Curved`; it also rejects a fitted sheet whose area factor degenerates as
`Folded`. These are more specific native targets than the preview's mesh diagnostic. The enum
alone does not identify which geometric event caused a particular `Curved` refusal, so the table
does not claim a proven swallowtail or other singularity.

The existing `swept/tumbling-torus` still grades **built**, with STEP classified as `brep`; its
kernel validity flag was false, which the current benchmark reports but does not grade.
All ten additions grade **refused**, not crashed or timed out. OCCT explicitly refuses all ten
for its missing continuous solid-envelope operation. No grading rules or kernel code changed.

## Reproduce

Run the benchmark implementations and the existing baseline:

```sh
python3 harness/build.py cad-kernel
python3 harness/run.py --kernel cad-kernel --case swept/tumbling-torus --timeout 120
python3 harness/run.py --kernel occt --case swept/tumbling-torus- --timeout 30
```

These are subset runs; they do not overwrite the published result tables. The full suite's case
loader parsed all 346 descriptions, including the ten additions. Both adapters were exercised,
and the Rust adapter was compiled against its pinned release.
