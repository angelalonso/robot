#[derive(Debug, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub x_pos: i32,
    pub y_pos: i32,
    pub nw: Coord,
    pub ne: Coord,
    pub sw: Coord,
    pub se: Coord,
    pub air: bool,
}

impl Area {
    #[allow(dead_code)]
    pub fn new(x_pos: i32, y_pos: i32, nw: Coord, ne: Coord, sw: Coord, se: Coord, air: bool) -> Area {
        Area { x_pos, y_pos, nw, ne, sw, se, air }
    }
}


#[derive(Debug, Clone)]
pub struct Datapoint {
    pub pos: Coord,
    pub solid: bool,
}

impl Datapoint {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32, solid: bool) -> Datapoint {
        let pos = Coord {
            x,
            y
        };
        Datapoint { pos, solid }
    }
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub set: Vec<Datapoint>,
    pub mapxy: Vec<Area>,
    pub min_angle: i32,
    pub max_angle: i32,
    pub max_distance: i32,
    pub max_distance_graphic: i32,

}

impl Dataset {
    #[allow(dead_code)]
    pub fn new(min_angle: i32, max_angle: i32, max_distance: i32, max_distance_graphic: i32) -> Dataset {
        Dataset { 
            set: [].to_vec(),
            mapxy: [].to_vec(),
            min_angle: min_angle,
            max_angle: max_angle,
            max_distance: max_distance,
            max_distance_graphic: max_distance_graphic,
        }
    }
    // TODO: clean up whole library, use it on ROS
    #[allow(dead_code)]
    pub fn add(&mut self, dp: Datapoint) {
        self.set.append(&mut [dp].to_vec())
    }

    // given angle and distance,
    //         generate a set of points in straight line, all solid=false but the one at the given
    //         distance
    pub fn add_ping(&mut self, angle_raw: i32, distance: i32) -> Coord {
        // transform angle to degrees (-90 to 90)
        let angle = ((angle_raw - self.min_angle) * 180 / (self.max_angle - self.min_angle)) - 90;
        println!("  Angle {:?}", angle);
        // find point, given angle and distance
        let origin = Coord {
            x: 0,
            y: 0,
        };
        let hit = Coord {
            x: (distance as f64 * f64::from(angle).sin()) as i32,
            y: (distance as f64 * f64::from(angle).cos()) as i32,
        };
        for y in 0..self.max_distance_graphic {
            for x in 0..self.max_distance_graphic * 2 {
                for entry in self.mapxy.clone() {
                    if entry.x_pos == x && entry.y_pos == y {
                        if (self.newcross(origin.clone(), hit.clone(), entry.sw.clone(), entry.se.clone())) ||
                           (self.newcross(origin.clone(), hit.clone(), entry.se.clone(), entry.ne.clone())) ||
                           (self.newcross(origin.clone(), hit.clone(), entry.ne.clone(), entry.nw.clone())) ||
                           (self.newcross(origin.clone(), hit.clone(), entry.nw.clone(), entry.sw.clone())) {
                               self.set_air(x, y, true);
                               //println!("HIT found between {:?} - {:?} and {:?} ", origin, hit, entry)
                        }
                    }
                }
            }
        }
        self.gethit(hit.clone());
        // find quadrants on that angle
        // mark all quadrants as solid=False but the one on distance=distance
        println!("-----------------     {:?}", hit);
        hit
    }

    pub fn set_air(&mut self, x: i32, y: i32, air: bool) {
        for mut entry in &mut self.mapxy {
            if entry.x_pos == x && entry.y_pos == y {
                entry.air = air;
            }
        }
    }
    /* -------------- ping aux functions ---------------------------*/
    pub fn gethit(&mut self, a: Coord) {
        for mut entry in &mut self.mapxy {
            let mut x_in = false;
            let mut y_in = false;
            let x_nesw = entry.ne.x..entry.sw.x;
            let x_swne = entry.sw.x..entry.ne.x;
            let y_nesw = entry.ne.y..entry.sw.y;
            let y_swne = entry.sw.y..entry.ne.y;
            if (x_nesw.contains(&a.x)) || (x_swne.contains(&a.x)) { x_in = true; };
            if (y_nesw.contains(&a.y)) || (y_swne.contains(&a.y)) { y_in = true; };
            if x_in && y_in {
                println!("HIT found AT {:?} - {:?} ", entry, a);
                entry.air = false
            };
        };
    }

    pub fn substract(&mut self, a: Coord, b: Coord) -> Coord {
        let result = Coord {
            x: a.x - b.x,
            y: a.y - b.y,
        };
        result
    }
    pub fn cross_product(&mut self, a: Coord, b: Coord) -> i32 {
        return a.x * b.y - b.x * a.y
    }

    pub fn direction(&mut self, a: Coord, b: Coord, c: Coord) -> i32 {
        let s1 = self.substract(c, a.clone());
        let s2 = self.substract(b, a.clone());
        let result = self.cross_product(s1, s2);
        return result
    }

    pub fn on_segment(&mut self, a: Coord, b: Coord, c: Coord) -> bool {
        return std::cmp::min(a.x, b.x) <= c.x && c.x <= std::cmp::max(a.x, b.x) && std::cmp::min(a.y, b.y) <= c.y && c.y <= std::cmp::max(a.y, b.y)

    }
    pub fn newcross(&mut self, a: Coord, b: Coord, c: Coord, d: Coord) -> bool {
        let d1 = self.direction(c.clone(), d.clone(), a.clone());
        let d2 = self.direction(c.clone(), d.clone(), b.clone());
        let d3 = self.direction(a.clone(), b.clone(), c.clone());
        let d4 = self.direction(a.clone(), b.clone(), d.clone());
        if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)) {
                return true
            } else if d1 == 0 && self.on_segment(c.clone(), d.clone(), a.clone()) {
                return true
            } else if d2 == 0 && self.on_segment(c.clone(), d.clone(), b.clone()) {
                return true
            } else if d3 == 0 && self.on_segment(a.clone(), b.clone(), c) {
                return true
            } else if d4 == 0 && self.on_segment(a, b, d) {
                return true
            } else {
                return false
            }
    }
    /*
def intersect(p1, p2, p3, p4):
        d1 = direction(p3, p4, p1)
    d2 = direction(p3, p4, p2)
    d3 = direction(p1, p2, p3)
    d4 = direction(p1, p2, p4)

    if ((d1 > 0 and d2 < 0) or (d1 < 0 and d2 > 0)) and \
        ((d3 > 0 and d4 < 0) or (d3 < 0 and d4 > 0)):
        return True

    elif d1 == 0 and on_segment(p3, p4, p1):
        return True
    elif d2 == 0 and on_segment(p3, p4, p2):
        return True
    elif d3 == 0 and on_segment(p1, p2, p3):
        return True
    elif d4 == 0 and on_segment(p1, p2, p4):
        return True
    else:
        return False
    */
    /* -------------- test ---------------------------*/

    #[allow(dead_code)]
    pub fn cross(&mut self, a1: Coord, a2: Coord, b1: Coord, b2: Coord) -> bool {
        let r1 = a2.y - a1.y;
        let s1 = a1.x - a2.x;
        let t1 = r1 * a1.x + s1 * a1.y;
 
        let r2 = b2.y - b1.y;
        let s2 = b1.x - b2.x;
        let t2 = r2 * b1.x + s2 * b1.y;
 
        let delta = (r1 * s2 - r2 * s1) as f32;
 
        if delta == 0.0 {
            return false;
        }
 
        let result = Some(Coord {
            x: ((s2 * t1 - s1 * t2) as f32 / delta) as i32,
            y: ((r1 * t2 - r2 * t1) as f32 / delta) as i32,
        });
        if result.is_none() {
            false
        } else {
            println!("--------------- HIT AT {:?},{:?} vs {:?},{:?}", a1, a2, b1, b2);
            true
        }
    }

    pub fn show(&self) -> Vec<String> {
        let mut result = Vec::new();
        for y in (0..self.max_distance_graphic).rev() {
            let mut x_string = "".to_string();
            for x in 0..self.max_distance_graphic * 2 {
                for entry in self.mapxy.clone() {
                    if entry.x_pos == x && entry.y_pos == y {
                        if entry.air {
                            x_string.push_str("."); 
                        } else {
                            x_string.push_str("#"); 
                        }
                    }
                }
            }
            result.push(x_string);
        }
        result
    }

    #[allow(dead_code)]
    pub fn old_show(&self) -> Vec<String> {
        let mut result = Vec::new();
        for x in 1..(self.max_distance_graphic + 1) {
            let mut line = "".to_owned();
            for y in 1..((2 * self.max_distance_graphic) + 1) {
                let mut object = false;
                for dp in self.set.clone() {
                    if dp.pos.x == x && dp.pos.y == y && dp.solid {
                        object = true;
                    }
                }
                if object {
                    line.push_str("O");
                } else {
                    line.push_str(".");
                }
            }
            result.push(line);
        }
        result
    }

    pub fn build_map(&mut self) {
        self.mapxy.clear();
        let side = self.max_distance / self.max_distance_graphic;
        for x in (0 - self.max_distance_graphic)..self.max_distance_graphic {
            for y in 0..self.max_distance_graphic {
                let new_sw = Coord {
                    x: (side * x), 
                    y: (side * y), 
                }; 
                let new_se = Coord {
                    x: (side * x), 
                    y: (side * y) + side -1,
                };
                let new_nw = Coord {
                    x: (side * x) + side -1, 
                    y: side * y,
                };
                let new_ne = Coord {
                    x: (side * x) + side -1, 
                    y: (side * y) + side -1
                };
                let sq = Area {
                    x_pos: x + self.max_distance_graphic,
                    y_pos: y,
                    sw: new_sw,
                    se: new_se,
                    nw: new_nw,
                    ne: new_ne,
                    air: true,
                };
                self.mapxy.push(sq.clone());
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_map_as_str(&self) -> Vec<String> {
        let mut result = [].to_vec();
        for line in self.mapxy.to_owned() {
            let mut linestring = [format!("{:?}", line).to_owned()].to_vec();
            result.append(&mut linestring);
        }
        result
    }
}

///// A Python module implemented in Rust.
//fn rustbrain(_py: Python, m: &PyModule) -> PyResult<()> {
//    m.add_class::<Coord>()?;
//    m.add_class::<Area>()?;
//    m.add_class::<Datapoint>()?;
//    m.add_class::<Dataset>()?;
//    Ok(())
//}


/*
#[test]
fn test_intersect() {
    let mut test_something = Dataset::new(500, 2500, 450, 2);
    test_something.build_map();
    let p1 = Coord {
        x: 0,
        y: 0
    };
    let p2 = Coord {
        x: 10,
        y: 0
    };
    let q1 = Coord {
        x: 10,
        y: 10
    };
    let q2 = Coord {
        x: 0,
        y: 0
    };
    let o2 = Coord {
        x: 2,
        y: 0
    };
    let o1 = Coord {
        x: 12,
        y: 0
    };
    println!("- cross, crossed lines");
    assert_eq!(test_something.cross(p1.clone(), p2.clone(), q1.clone(), q2.clone()), true);
    println!("- cross, crossed lines2");
    assert_eq!(test_something.cross(o1.clone(), o2.clone(), q1.clone(), q2.clone()), true);
    println!("- cross, crossed lines3");
    assert_eq!(test_something.cross(o1.clone(), o2.clone(), p1.clone(), p2.clone()), false);
}

#[test]
fn test_ping() {
    let mut test_something = Dataset::new(500, 2500, 450, 2);
    test_something.build_map();
    test_something.add_ping(500, 225);
    let expected = [
        "....", 
        "....", 
    ];
    assert_eq!(test_something.show(), expected);
}
#[test]
fn test_data() {
    let mut test_something = Dataset::new(500, 2500, 450, 2);
    test_something.build_map();
    let map_expected = [
"Area { x_pos: 2, y_pos: 0, nw: Coord { x: 224, y: 0 }, ne: Coord { x: 224, y: 224 }, sw: Coord { x: 0, y: 0 }, se: Coord { x: 0, y: 224 }, air: true }", "Area { x_pos: 2, y_pos: 1, nw: Coord { x: 224, y: 225 }, ne: Coord { x: 224, y: 449 }, sw: Coord { x: 0, y: 225 }, se: Coord { x: 0, y: 449 }, air: true }", "Area { x_pos: 3, y_pos: 0, nw: Coord { x: 449, y: 0 }, ne: Coord { x: 449, y: 224 }, sw: Coord { x: 225, y: 0 }, se: Coord { x: 225, y: 224 }, air: true }", "Area { x_pos: 3, y_pos: 1, nw: Coord { x: 449, y: 225 }, ne: Coord { x: 449, y: 449 }, sw: Coord { x: 225, y: 225 }, se: Coord { x: 225, y: 449 }, air: true }", "Area { x_pos: 0, y_pos: 0, nw: Coord { x: -225, y: 0 }, ne: Coord { x: -225, y: 224 }, sw: Coord { x: -1, y: 0 }, se: Coord { x: -1, y: 224 }, air: true }", "Area { x_pos: 0, y_pos: 1, nw: Coord { x: -225, y: 225 }, ne: Coord { x: -225, y: 449 }, sw: Coord { x: -1, y: 225 }, se: Coord { x: -1, y: 449 }, air: true }", "Area { x_pos: 1, y_pos: 0, nw: Coord { x: -450, y: 0 }, ne: Coord { x: -450, y: 224 }, sw: Coord { x: -226, y: 0 }, se: Coord { x: -226, y: 224 }, air: true }", "Area { x_pos: 1, y_pos: 1, nw: Coord { x: -450, y: 225 }, ne: Coord { x: -450, y: 449 }, sw: Coord { x: -226, y: 225 }, se: Coord { x: -226, y: 449 }, air: true }"
    ];

    assert_eq!(test_something.show_map_as_str(), map_expected);
}
#[test]
fn test_coords() {
    let mut test_something = Dataset::new(500, 2500, 450, 2);
    test_something.build_map();
    let expected = Coord {
        x: 0,
        y: 4,
    };
    let result = test_something.add_ping(1500, 4);
    let gotshow = test_something.show();
//    assert_eq!(result.x, expected.x);
//    assert_eq!(result.y, expected.y);
    let expectedshow = [
        "....", 
        "....", 
    ];
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
//    assert_eq!(test_something.show(), expectedshow);
}
*/
#[test]
fn test_shot() {
    let mut test_something = Dataset::new(500, 2500, 450, 20);
    test_something.build_map();
    println!("-------------------------------------------------");
    test_something.add_ping(1500, 50);
    let mut gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(1500, 75);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(1500, 100);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(500, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(1000, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(1350, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(1500, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(1650, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2150, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2500, 190);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 225);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 275);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 325);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 375);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 425);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 475);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
    println!("-------------------------------------------------");
    test_something.add_ping(2000, 525);
    gotshow = test_something.show();
    for i in gotshow.clone() {
        println!("|{:?}|", i);
    }
//    assert_eq!(test_something.show(), expectedshow);
}