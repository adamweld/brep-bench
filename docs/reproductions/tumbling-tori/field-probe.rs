use gcs_core::{library,program,solve,solid::{MaterialField,FieldMesher,Query,Source,Resolution},space::box_centre_diagonal};
fn main() {
    let args:Vec<_>=std::env::args().collect();
    let text=std::fs::read_to_string(&args[1]).unwrap();
    let (p,errs,diags)=library::parse_linked(&text);
    assert!(errs.is_empty() && diags.is_empty(),"{errs:?} {diags:?}");
    let mut e=program::elaborate(&p);assert!(e.ok(),"{:?}",e.diags);
    assert!(solve::solve(&mut e.sketch,Default::default()).success);
    let i=e.map.ent_named(&args[2]).unwrap().i();
    let fine:f64=args[3].parse().unwrap();
    let field=MaterialField::read(&e.sketch,i,1e-10).unwrap();
    let support=field.support_bounds().unwrap().unwrap();
    let (centre,diag)=box_centre_diagonal(&support);
    println!("support={support:?} diagonal={diag} centre={centre:?}");
    // The centre of the tube at rest is strictly inside every source sweep whose interval includes zero.
    // User supplies it explicitly so the probe makes no guesses about the document's geometry.
    let witness:[f64;3]=[args[4].parse().unwrap(),args[5].parse().unwrap(),args[6].parse().unwrap()];
    let exact=field.query(witness,&mut Query::at(witness));
    println!("witness={witness:?} exact={}",exact.value);
    for f in [1.,2.,4.] {
        let res=Resolution {coarsest:diag/(40.*f),finest:diag/(1000.*f*f),tolerance:diag/(20000.*f*f)};
        let mut q=Query {source:Source::Cached(res),..Query::at(witness)};
        let value=field.query(witness,&mut q).value;
        println!("cached fineness={f} resolution={:?} value={value}",q.source);
    }
    let mut mesher=FieldMesher::with_fineness(&e.sketch,i,fine).unwrap();
    let mut last=String::new();
    loop {
        let pr=mesher.progress();let now=format!("{} {:?} rebuild {} curves {:?}",pr.phase,pr.refine.stage,pr.refine.rebuild,pr.curves);
        if now!=last {let s=mesher.snapshot();println!("{now} vertices={} triangles={} progress={:?}",s.vertices.len(),s.triangles.len(),pr.refine);last=now;}
        match mesher.step(500) {
            Ok(false)=>{},
            Ok(true)=>{let s=mesher.snapshot();println!("DONE vertices={} triangles={} provisional={}",s.vertices.len(),s.triangles.len(),s.provisional);break;},
            Err(why)=>{println!("FAILED {why}");break;}
        }
    }
}
