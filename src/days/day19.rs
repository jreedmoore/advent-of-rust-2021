mod puzzle {
  use nalgebra as na;
  use petgraph::{
    graphmap::{DiGraphMap},
    visit::{depth_first_search, DfsEvent, Control}
  };
  use std::collections::HashSet;

  type Point3f = na::Point3<f32>;
  type Isometry3f = na::Isometry3<f32>;

  #[derive(Hash, PartialEq, Eq)]
  pub struct HashedPoint3 {
    x: i32,
    y: i32,
    z: i32,
  }
  impl HashedPoint3 {
    fn new(p: Point3f) -> HashedPoint3 {
      todo!();
    }
  }

  pub struct ScannerInput {
    id: usize,
    beacon_relative_locations: Vec<Point3f>
  }

  pub struct Map {
    pub beacons: HashSet<HashedPoint3>,
    pub scanners: HashSet<HashedPoint3>,
  }

  fn likely_pairs(input: &[ScannerInput]) -> Vec<(ScannerInput, ScannerInput)> {
    todo!()
  }

  fn scanner_graph(pairs: Vec<(ScannerInput, ScannerInput)>) -> DiGraphMap<usize, Isometry3f> {
    todo!()
  }

  fn scanner_pos(graph: &DiGraphMap<usize, Isometry3f>) -> HashSet<HashedPoint3> {
    let mut isometry_stack: Vec<Isometry3f> = Vec::new();
    let mut scanners: HashSet<HashedPoint3> = HashSet::new();
    scanners.insert(HashedPoint3::new(Point3f::origin()));
    let _: Control<()> = depth_first_search(&graph, Some(0), |event| {
      match event {
        DfsEvent::TreeEdge(u, v) => { 
          isometry_stack.push(graph.edge_weight(u,v).unwrap().clone());
          let transformed: Point3f = isometry_stack.iter().fold(Point3f::origin(), |p, iso| *iso * p);
          scanners.insert(HashedPoint3::new(transformed));
        },
        DfsEvent::Finish(_, _) => { isometry_stack.pop(); () },
        _ => (),
      }
      Control::Continue
    });
    scanners
  }

  fn beacon_pos(graph: &DiGraphMap<usize, Isometry3f>, input: &[ScannerInput]) -> HashSet<HashedPoint3> {
    let mut isometry_stack: Vec<Isometry3f> = Vec::new();
    let mut beacons: HashSet<HashedPoint3> = HashSet::new();
    let _: Control<()> = depth_first_search(&graph, Some(0), |event| {
      match event {
        DfsEvent::TreeEdge(u, v) => { 
          isometry_stack.push(graph.edge_weight(u,v).unwrap().clone());
          for beacon in &input[v].beacon_relative_locations {
            let transformed: Point3f = isometry_stack.iter().fold(beacon.clone(), |p, iso| *iso * p);
            beacons.insert(HashedPoint3::new(transformed));
          }
        },
        DfsEvent::Finish(_, _) => { isometry_stack.pop(); () },
        _ => (),
      }
      Control::Continue
    });
    beacons
  }

  pub fn build_map(input: &[ScannerInput]) -> Map {
    // lots of inspiration from this thread: https://www.reddit.com/r/adventofcode/comments/rjpf7f/2021_day_19_solutions/

    // find likely pairs by computing and comparing distances
    let pairs = likely_pairs(input);

    // use an algorithm like Umeyama to align https://zpl.fi/aligning-point-patterns-with-kabsch-umeyama-algorithm/
    // (our problem is less general, since we don't need to support scaling, just translation and rotation, and our rotation is always aligned to axis)
    // maintain a graph from scanner 0 to other scanners to compute final points, edges are Isometry (rotation + translation) between scanners
    let graph = scanner_graph(pairs);

    // walk graph from scanner_0 to build unique sets of scanners and beacons
    let scanners = scanner_pos(&graph);
    let beacons = beacon_pos(&graph, input);

    Map { scanners: scanners, beacons: beacons }
  }
  pub fn parse(input: &str) -> Option<Vec<ScannerInput>> {
    todo!()
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_likely_pairs() {
      let (l, r) = &likely_pairs(&parse(EXAMPLE).unwrap())[0];
      assert_eq!(l.id, 0);
      assert_eq!(r.id, 1);
    }
  }

  #[cfg(test)]
  pub const EXAMPLE: &'static str = r#"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
  "#;
}

pub fn part_one(input: &str) -> Option<u64> {
  puzzle::parse(input).map(|scanner_input| puzzle::build_map(&scanner_input).beacons.len() as u64)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one_example() {
    assert_eq!(part_one(puzzle::EXAMPLE), Some(79));
  }
}