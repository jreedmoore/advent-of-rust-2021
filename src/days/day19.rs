mod puzzle {
  use nalgebra as na;
  use petgraph::{
    graphmap::{DiGraphMap},
    visit::{depth_first_search, DfsEvent, Control}
  };
  use std::collections::HashSet;
  use itertools::Itertools;

  use crate::util::intersect::count_intersect;

  type Vec3f = na::Vector3<f32>;
  type Point3f = na::Point3<f32>;
  type Isometry3f = na::Isometry3<f32>;

  #[derive(Debug, Hash, PartialEq, Eq)]
  pub struct HashedPoint3 {
    x: i32,
    y: i32,
    z: i32,
  }
  impl HashedPoint3 {
    fn new(p: &Point3f) -> HashedPoint3 {
      HashedPoint3 {
        x: p.coords.x.round() as i32,
        y: p.coords.y.round() as i32,
        z: p.coords.z.round() as i32,
      }
    }
    fn set_from_slice(ps: &[Point3f]) -> HashSet<HashedPoint3> {
      ps.iter().map(|p| HashedPoint3::new(p)).collect()
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


  pub struct CorrespondingPairs {
    a: usize,
    b: usize,
    point_distances: HashSet<i32>,
  }

  fn likely_pairs(input: &[ScannerInput]) -> Vec<CorrespondingPairs> {
    let distance_sets: Vec<HashSet<i32>> = 
      input.iter()
        .map(|input| input.beacon_relative_locations.iter().tuple_combinations().map(|(a,b)| (a-b).norm() as i32).collect::<HashSet<i32>>())
        .collect();

    distance_sets.iter().enumerate().tuple_combinations()
      .filter_map(|((li, ls),(ri, rs))| {
        let intersection_set = ls.intersection(rs).cloned().collect::<HashSet<i32>>();
        let intersection_size = intersection_set.len();
        // 12 choose 2 == 66
        if intersection_size >= 66 && li != ri {
          Some(CorrespondingPairs { a: li, b: ri, point_distances: intersection_set })
        } else {
          None
        }
      })
      .collect()
  }

  fn centroid(ps: &[Point3f]) -> Point3f {
    let v: Vec3f = ps.iter().fold(na::zero(), |acc, p| acc + p.coords);
    Point3f::from(v / (ps.len() as f32))
  }

  // produce Isometry aligning points of a onto point of b
  fn align_points(a: &[Point3f], b: &[Point3f]) -> Isometry3f {
    // https://zpl.fi/aligning-point-patterns-with-kabsch-umeyama-algorithm/
    if a.len() != b.len() {
      // assuming that a[i] maps to b[i]
      // (mapping needs to be established before this call!)
      panic!("align_points requires point sets to match in length");
    }
    let n = a.len();
    let centroid_a = centroid(a);
    let centroid_b = centroid(b);

    // Most descriptions of this algorithm are written in terms of data in rows,
    // but nalgebra::geometry makes data in rows more convenient
    let a_variance = na::Matrix3xX::from_columns(&a.iter().map(|pa| pa - centroid_a).collect_vec());
    let b_variance = na::Matrix3xX::from_columns(&b.iter().map(|pb| pb - centroid_b).collect_vec());

    let covariances = a_variance * b_variance.transpose() / (n as f32);
    let svd = na::linalg::SVD::new(covariances.clone(), true, true);
    let u = svd.u.unwrap();
    let d = svd.singular_values;
    let v_t = svd.v_t.unwrap();

    let d = f32::signum(u.determinant() * v_t.determinant());
    let s = na::Matrix3::from_diagonal(&Vec3f::new(1.0, 1.0, d));

    let r = u*s*v_t;
    let rotation = na::Rotation3::from_matrix(&r.fixed_slice::<3,3>(0,0).into());
    let translation = na::Translation3::from(centroid_a - (rotation * centroid_b));

    Isometry3f::from_parts(translation, na::UnitQuaternion::from_rotation_matrix(&rotation))
  }

  fn distance_matrix(ps: &[Point3f]) -> na::DMatrix<i32> {
    let n = ps.len();
    let mut mat = na::DMatrix::zeros(n, n);
    for i in 0..n {
      for j in 0..n {
        mat[(i,j)] = (&ps[i] - &ps[j]).norm().round() as i32;
      }
    }
    mat
  }

  fn corresponding_points(a: &[Point3f], b: &[Point3f]) -> Vec<(Point3f, Point3f)> {
    // compute for a and b dist(u,v)
    let a_dist = distance_matrix(a);
    let b_dist = distance_matrix(b);

    let mut pairs: Vec<(Point3f, Point3f)> = Vec::new();
    let mut b_used: HashSet<usize> = HashSet::new();
    for (ia, pa) in a.iter().enumerate() {
      for (ib, pb) in b.iter().enumerate() {
        if b_used.contains(&ib) { continue; }
        let ac = a_dist.column(ia).iter().cloned().sorted().collect_vec();
        let bc = b_dist.column(ib).iter().cloned().sorted().collect_vec();

        let possible_matches = std::cmp::min(ac.len(), bc.len());
        let matches = count_intersect(&ac, &bc);

        if matches > possible_matches / 2 || matches >= 12 {
          b_used.insert(ib);
          pairs.push((pa.clone(),pb.clone()));
          break;
        }
      }
    }

    pairs
  }

  fn align_pair(pair: &CorrespondingPairs, input_a: &[Point3f], input_b: &[Point3f]) -> Option<Isometry3f> {
    // filter corresponding points. if <= 12 return None
    let a_ps =
      input_a.iter()
        .filter(|a| {
          input_b.iter().filter(|b| pair.point_distances.contains(&((*a-*b).norm() as i32))).count() >= 12
        })
        .cloned()
        .collect_vec();
    let b_ps =
      input_b.iter()
        .filter(|b| {
          input_a.iter().filter(|a| pair.point_distances.contains(&((*a-*b).norm() as i32))).count() >= 12
        })
        .cloned()
        .collect_vec();

    if a_ps.len() >= 12 {
      println!("a {:?} b {:?}", a_ps.len(), b_ps.len());
      Some(align_points(&a_ps, &b_ps))
    } else {
      None
    }
  }

  fn scanner_graph(pairs: Vec<CorrespondingPairs>, input: &[ScannerInput]) -> DiGraphMap<usize, Isometry3f> {
    // for each likely pair
    // filter corresponding points from pair to produce P and Q point sets
    // SVD to find rotation and translation matrix (gnarly iterative process)
    //   https://zpl.fi/aligning-point-patterns-with-kabsch-umeyama-algorithm/
    //   https://igl.ethz.ch/projects/ARAP/svd_rot.pdf 
    // are there any examples of how this might fail? what would that look like?
    // maybe compute points after transform and verify 12 or more overlaps (in i32?)
    todo!()
  }

  fn scanner_pos(graph: &DiGraphMap<usize, Isometry3f>) -> HashSet<HashedPoint3> {
    let mut isometry_stack: Vec<Isometry3f> = Vec::new();
    let mut scanners: HashSet<HashedPoint3> = HashSet::new();
    scanners.insert(HashedPoint3::new(&Point3f::origin()));
    let _: Control<()> = depth_first_search(&graph, Some(0), |event| {
      match event {
        DfsEvent::TreeEdge(u, v) => { 
          isometry_stack.push(graph.edge_weight(u,v).unwrap().clone());
          let transformed: Point3f = isometry_stack.iter().fold(Point3f::origin(), |p, iso| *iso * p);
          scanners.insert(HashedPoint3::new(&transformed));
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
            beacons.insert(HashedPoint3::new(&transformed));
          }
        },
        DfsEvent::Finish(_, _) => { isometry_stack.pop(); () },
        _ => (),
      }
      Control::Continue
    });
    beacons
  }

  // assumes input is sorted, i.e. input[i].id == i
  pub fn build_map(input: &[ScannerInput]) -> Map {
    // lots of inspiration from this thread: https://www.reddit.com/r/adventofcode/comments/rjpf7f/2021_day_19_solutions/

    // find likely pairs by computing and comparing distances
    let pairs = likely_pairs(input);

    // use an algorithm like Umeyama to align https://zpl.fi/aligning-point-patterns-with-kabsch-umeyama-algorithm/
    // (our problem is less general, since we don't need to support scaling, just translation and rotation, and our rotation is always aligned to axis)
    // maintain a graph from scanner 0 to other scanners to compute final points, edges are Isometry (rotation + translation) between scanners
    let graph = scanner_graph(pairs, input);

    // walk graph from scanner_0 to build unique sets of scanners and beacons
    let scanners = scanner_pos(&graph);
    let beacons = beacon_pos(&graph, input);

    Map { scanners: scanners, beacons: beacons }
  }
  pub fn parse(input: &str) -> Option<Vec<ScannerInput>> {
    parser::puzzle_input(input).ok().map(|t| t.1)
  }
  mod parser {
    use super::*;
    use nom::{
      IResult,
      error::ParseError,
      combinator::map,
      sequence::{terminated, tuple, delimited},
      multi::many1,
      character::complete::{char, multispace0},
      bytes::complete::tag
    };
    /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and 
    /// trailing whitespace, returning the output of `inner`.
    fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
      where
      F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
      delimited(
        multispace0,
        inner,
        multispace0
      )
    }
    fn scanner_line(input: &str) -> IResult<&str, usize> {
      map(
        delimited(
          ws(tag("--- scanner")),
          nom::character::complete::u64,
          ws(tag("---"))
        ),
        |id| id as usize
      )(input)
    }
    fn beacon_position(input: &str) -> IResult<&str, Point3f> {
      map(
        tuple((
          terminated(nom::character::complete::i32, char(',')),
          terminated(nom::character::complete::i32, char(',')),
          nom::character::complete::i32,
       )),
       |(x,y,z)| Point3f::new(x as f32, y as f32, z as f32)
      )(input)
    }
    fn scanner_input(input: &str) -> IResult<&str, ScannerInput> {
      map(
        tuple((
          ws(scanner_line),
          many1(ws(beacon_position))
        )), 
        |(id, beacons)| ScannerInput {id:id, beacon_relative_locations: beacons}
      )(input)
    }
    pub fn puzzle_input(input: &str) -> IResult<&str, Vec<ScannerInput>> {
      many1(ws(scanner_input))(input)
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_likely_pairs() {
      let pairs = likely_pairs(&parser::puzzle_input(EXAMPLE).unwrap().1);
      assert_eq!(pairs.iter().map(|cp| (cp.a, cp.b)).collect::<Vec<(usize,usize)>>(), vec![(0,1), (1,3), (1,4), (2,4)]);
    }

    #[test]
    fn test_align_pair() {
      let input = parse(EXAMPLE).unwrap();
      let pairs = likely_pairs(&input);
      let isometry = align_pair(&pairs[0], &input[0].beacon_relative_locations, &input[1].beacon_relative_locations).unwrap();

      assert_eq!(HashedPoint3::new(&(isometry * Point3f::origin())), HashedPoint3::new(&Point3f::new(68.0,-1246.0,-43.0)));
    }

    #[test]
    fn test_corresponding_points() {
      let input = parse(EXAMPLE).unwrap();
      let pairs = corresponding_points(&input[0].beacon_relative_locations, &input[1].beacon_relative_locations);
      let (a,b): (Vec<_>, Vec<_>) = pairs.iter().cloned().unzip();

      assert_eq!(HashedPoint3::set_from_slice(&a), HashedPoint3::set_from_slice(&vec![
        Point3f::new(-618.0,-824.0,-621.0),
        Point3f::new(-537.0,-823.0,-458.0),
        Point3f::new(-447.0,-329.0,318.0),
        Point3f::new(404.0,-588.0,-901.0),
        Point3f::new(544.0,-627.0,-890.0),
        Point3f::new(528.0,-643.0,409.0),
        Point3f::new(-661.0,-816.0,-575.0),
        Point3f::new(390.0,-675.0,-793.0),
        Point3f::new(423.0,-701.0,434.0),
        Point3f::new(-345.0,-311.0,381.0),
        Point3f::new(459.0,-707.0,401.0),
        Point3f::new(-485.0,-357.0,347.0),
      ]));
      assert_eq!(HashedPoint3::set_from_slice(&b), HashedPoint3::set_from_slice(&vec![
        Point3f::new(686.0,422.0,578.0),
        Point3f::new(605.0,423.0,415.0),
        Point3f::new(515.0,917.0,-361.0),
        Point3f::new(-336.0,658.0,858.0),
        Point3f::new(-476.0,619.0,847.0),
        Point3f::new(-460.0,603.0,-452.0),
        Point3f::new(729.0,430.0,532.0),
        Point3f::new(-322.0,571.0,750.0),
        Point3f::new(-355.0,545.0,-477.0),
        Point3f::new(413.0,935.0,-424.0),
        Point3f::new(-391.0,539.0,-444.0),
        Point3f::new(553.0,889.0,-390.0),
      ]))
    }

    #[test]
    fn test_corresponding_points_translated() {
      let a = vec![
        Point3f::new(0.0, 2.0, 0.0),
        Point3f::new(4.0, 1.0, 0.0),
        Point3f::new(3.0, 3.0, 0.0),
      ];
      let translation = na::Translation3::from(Vec3f::new(-5.0,-2.0,0.0));
      let b = a.iter().map(|p| translation * p).collect_vec();

      let (aa, bb): (Vec<_>, Vec<_>) = corresponding_points(&a, &b).iter().cloned().unzip();

      assert_eq!(aa, a);
      assert_eq!(bb, b);
    }

    #[test]
    fn test_align_points_example() {
        let a = vec![
        Point3f::new(-618.0,-824.0,-621.0),
        Point3f::new(-537.0,-823.0,-458.0),
        Point3f::new(-447.0,-329.0,318.0),
        Point3f::new(404.0,-588.0,-901.0),
        Point3f::new(544.0,-627.0,-890.0),
        Point3f::new(528.0,-643.0,409.0),
        Point3f::new(-661.0,-816.0,-575.0),
        Point3f::new(390.0,-675.0,-793.0),
        Point3f::new(423.0,-701.0,434.0),
        Point3f::new(-345.0,-311.0,381.0),
        Point3f::new(459.0,-707.0,401.0),
        Point3f::new(-485.0,-357.0,347.0),
      ];
      let b = vec![
        Point3f::new(686.0,422.0,578.0),
        Point3f::new(605.0,423.0,415.0),
        Point3f::new(515.0,917.0,-361.0),
        Point3f::new(-336.0,658.0,858.0),
        Point3f::new(-476.0,619.0,847.0),
        Point3f::new(-460.0,603.0,-452.0),
        Point3f::new(729.0,430.0,532.0),
        Point3f::new(-322.0,571.0,750.0),
        Point3f::new(-355.0,545.0,-477.0),
        Point3f::new(413.0,935.0,-424.0),
        Point3f::new(-391.0,539.0,-444.0),
        Point3f::new(553.0,889.0,-390.0),
      ];

      let isometry = align_points(&a, &b);
      assert_eq!(HashedPoint3::new(&(isometry * Point3f::origin())), HashedPoint3::new(&Point3f::new(68.0,-1246.0,-43.0)));
    }

    #[test]
    fn test_corresponding_points_translated_with_unique() {
      let a_common = vec![
        Point3f::new(0.0, 2.0, 0.0),
        Point3f::new(4.0, 1.0, 0.0),
        Point3f::new(3.0, 3.0, 0.0),
      ];
      let translation = na::Translation3::from(Vec3f::new(-5.0,-2.0,0.0));
      let b_common = a_common.iter().map(|p| translation * p).collect_vec();

      let mut a = a_common.clone();
      let mut b = b_common.clone();

      a.push(Point3f::new(-1.0,-1.0,-1.0));
      b.push(Point3f::new(-3.0,-3.0,-3.0));

      let (aa, bb): (Vec<_>, Vec<_>) = corresponding_points(&a, &b).iter().cloned().unzip();

      assert_eq!(aa, a_common);
      assert_eq!(bb, b_common);
    }

    #[test]
    fn test_align_points_translated() {
      let a = vec![
        Point3f::new(0.0, 2.0, 0.0),
        Point3f::new(4.0, 1.0, 0.0),
        Point3f::new(3.0, 3.0, 0.0),
      ];
      let b = vec![
        Point3f::new(-1.0, -1.0, 0.0),
        Point3f::new(-5.0, 0.0, 0.0),
        Point3f::new(-2.0, 1.0, 0.0),
      ];

      let isometry = align_points(&a, &b);
      assert_eq!(isometry * Point3f::origin(), Point3f::new(5.0, 2.0, 0.0));

      let scanner_0 = ScannerInput { id: 0, beacon_relative_locations: a.clone() };
      let scanner_1 = ScannerInput { id: 1, beacon_relative_locations: b.clone() };
      let input = vec![scanner_0, scanner_1];

      let mut graph: DiGraphMap<usize, Isometry3f> = DiGraphMap::new();
      graph.add_edge(0, 1, isometry);

      let scanners: HashSet<HashedPoint3> = vec![
        Point3f::new(0.0,0.0,0.0),
        Point3f::new(5.0,2.0,0.0)
      ].iter().map(|p| HashedPoint3::new(p)).collect();
      
      assert_eq!(scanner_pos(&graph), scanners);

      let beacons: HashSet<HashedPoint3> =
        a.iter().map(|p| HashedPoint3::new(p)).collect();

      assert_eq!(beacon_pos(&graph, &input), beacons);
    }
  }

  #[cfg(test)]
  pub const EXAMPLE: &'static str = include_str!("examples/day19-full.txt");
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