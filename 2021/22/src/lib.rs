use std::collections::HashSet;

pub fn get_input() -> String {
  String::from(include_str!("../input"))
}

pub fn get_sample_input() -> String {
  String::from(include_str!("../sample"))
}

pub fn solve_a(input: String) -> i64 {
  let steps = input
    .as_str()
    .replace("x=", "")
    .replace("y=", "")
    .replace("z=", "")
    .lines()
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<_>>();
      let on = match parts[0] {
        "on" => true,
        _ => false,
      };

      let ranges = parts[1]
        .split(",")
        .map(|range_str| {
          let range_vec = range_str
            .split("..")
            .map(|p| p.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

          (range_vec[0], range_vec[1])
        })
        .collect::<Vec<_>>();

      (on, ranges[0], ranges[1], ranges[2])
    })
    .collect::<Vec<_>>();

  let mut on_cubes: HashSet<(i64, i64, i64)> = HashSet::new();

  for (on, (x_min, x_max), (y_min, y_max), (z_min, z_max)) in steps {
    if x_min < -50 || x_max > 50 || y_min < -50 || y_max > 50 || z_min < -50 || z_max > 50 {
      continue;
    }

    for x in x_min..=x_max {
      for y in y_min..=y_max {
        for z in z_min..=z_max {
          let point = (x, y, z);

          if on {
            on_cubes.insert(point);
          } else {
            if on_cubes.contains(&point) {
              on_cubes.remove(&point);
            }
          }
        }
      }
    }
  }

  on_cubes.len() as i64
}

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
struct Cuboid2(Point, Point, Point);

impl Cuboid2 {
  fn new(x: Point, y: Point, z: Point) -> Self {
    Self(x, y, z)
  }

  fn intersection(&self, other: &Cuboid2) -> Option<Cuboid2> {
    let new = Cuboid2::new(
      (other.0 .0.max(self.0 .0), other.0 .1.min(self.0 .1)),
      (other.1 .0.max(self.1 .0), other.1 .1.min(self.1 .1)),
      (other.2 .0.max(self.2 .0), other.2 .1.min(self.2 .1)),
    );

    if !new.is_valid() {
      return None;
    }

    Some(new)
  }

  fn subtract(&self, other: &Cuboid2) -> Vec<Cuboid2> {
    if let Some(c) = self.intersection(other) {
      let mut sub_cuboids = Vec::new();

      for (i, x) in [
        (self.0 .0, c.0 .0 - 1),
        (c.0 .0, c.0 .1),
        (c.0 .1 + 1, self.0 .1),
      ]
      .iter()
      .enumerate()
      {
        for (j, y) in [
          (self.1 .0, c.1 .0 - 1),
          (c.1 .0, c.1 .1),
          (c.1 .1 + 1, self.1 .1),
        ]
        .iter()
        .enumerate()
        {
          for (k, z) in [
            (self.2 .0, c.2 .0 - 1),
            (c.2 .0, c.2 .1),
            (c.2 .1 + 1, self.2 .1),
          ]
          .iter()
          .enumerate()
          {
            if i == 1 && j == 1 && k == 1 {
              continue;
            }
            let new_cuboid = Cuboid2::new(*x, *y, *z);
            if new_cuboid.is_valid() {
              sub_cuboids.push(new_cuboid)
            }
          }
        }
      }
      return sub_cuboids;
    }

    vec![Cuboid2::new(self.0, self.1, self.2)]
  }

  fn is_valid(&self) -> bool {
    self.0 .1 - self.0 .0 >= 0 && self.1 .1 - self.1 .0 >= 0 && self.2 .1 - self.2 .0 >= 0
  }

  fn volume(&self) -> i64 {
    (self.0 .1 - self.0 .0 + 1) * (self.1 .1 - self.1 .0 + 1) * (self.2 .1 - self.2 .0 + 1)
  }
}

#[derive(Debug, Copy, Clone)]
struct Cuboid((i64, i64), (i64, i64), (i64, i64));
pub fn solve_b(input: String) -> i64 {
  let steps = input
    .as_str()
    .replace("x=", "")
    .replace("y=", "")
    .replace("z=", "")
    .lines()
    .map(|line| {
      let parts = line.split(" ").collect::<Vec<_>>();
      let on = match parts[0] {
        "on" => true,
        _ => false,
      };

      let ranges = parts[1]
        .split(",")
        .map(|range_str| {
          let range_vec = range_str
            .split("..")
            .map(|p| p.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

          (range_vec[0], range_vec[1])
        })
        .collect::<Vec<_>>();

      (on, Cuboid2::new(ranges[0], ranges[1], ranges[2]))
    })
    .collect::<Vec<_>>();

  let mut on_cubes: Vec<Cuboid2> = vec![];

  for (on, this) in steps {
    let mut next_on_cubes: Vec<_> = vec![];

    for cuboid in on_cubes {
      let sub_cubes = cuboid.subtract(&this);
      for sub_cube in sub_cubes {
        next_on_cubes.push(sub_cube);
      }
    }

    if on {
      next_on_cubes.push(this)
    }

    on_cubes = next_on_cubes;
  }

  on_cubes.iter().map(|c| c.volume()).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_eg_1() {
    assert_eq!(solve_a(get_sample_input()), 590784);
  }

  #[test]
  fn a() {
    assert_eq!(solve_a(get_input()), 581108);
  }

  #[test]
  fn b() {
    assert_eq!(solve_b(get_input()), 1325473814582641);
  }

  #[test]
  fn b_eg_1() {
    assert_eq!(
      solve_b(String::from(
        "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"
      )),
      2758514936282235
    )
  }

  //
}
