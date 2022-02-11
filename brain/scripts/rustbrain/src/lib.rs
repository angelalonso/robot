#![feature(macro_attributes_in_derive_output)]
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct Coord {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
}
#[pymethods]
impl Coord {
    #[new]
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}
#[derive(Debug, Clone)]
#[pyclass]
pub struct Area {
    pub x_pos: i32,
    pub y_pos: i32,
    #[pyo3(get, set)]
    pub nw: Coord,
    #[pyo3(get, set)]
    pub ne: Coord,
    #[pyo3(get, set)]
    pub sw: Coord,
    #[pyo3(get, set)]
    pub se: Coord,
    #[pyo3(get, set)]
    pub air: bool,
}
#[pymethods]
impl Area {
    #[new]
    pub fn new(x_pos: i32, y_pos: i32, nw: Coord, ne: Coord, sw: Coord, se: Coord, air: bool) -> Area {
        Area { x_pos, y_pos, nw, ne, sw, se, air }
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub struct Datapoint {
    pub pos: Coord,
    #[pyo3(get, set)]
    pub solid: bool,
}
#[pymethods]
impl Datapoint {
    #[new]
    pub fn new(x: i32, y: i32, solid: bool) -> Datapoint {
        let pos = Coord {
            x,
            y
        };
        Datapoint { pos, solid }
    }
}
#[derive(Debug, Clone)]
#[pyclass]
pub struct Dataset {
    #[pyo3(get, set)]
    pub set: Vec<Datapoint>,
    pub mapxy: Vec<Area>,
    pub min_angle: i32,
    pub max_angle: i32,
    pub max_distance: i32,
    pub max_distance_graphic: i32,

}
#[pymethods]
impl Dataset {
    #[new]
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
    pub fn add(&mut self, dp: Datapoint) {
        self.set.append(&mut [dp].to_vec())
    }

    // given angle and distance,
    //         generate a set of points in straight line, all solid=false but the one at the given
    //         distance
    pub fn add_ping(&mut self, angle_raw: i32, distance: i32) -> Coord {
        // transform angle to degrees (-90 to 90)
        let angle = ((angle_raw - self.min_angle) * 180 / (self.max_angle - self.min_angle)) - 90;
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
                        // TODO: troubleshoot this, add prints on each comparison
                        if (self.cross(origin.clone(), hit.clone(), entry.sw.clone(), entry.se.clone())) ||
                           (self.cross(origin.clone(), hit.clone(), entry.se.clone(), entry.ne.clone())) ||
                           (self.cross(origin.clone(), hit.clone(), entry.ne.clone(), entry.nw.clone())) ||
                           (self.cross(origin.clone(), hit.clone(), entry.nw.clone(), entry.sw.clone())) {
                               self.set_air(x, y, false);
                        }
                    }
                }
            }
        }

        // find quadrants on that angle
        // mark all quadrants as solid=False but the one on distance=distance
        hit
    }

    pub fn set_air(&mut self, x: i32, y: i32, air: bool) {
        for mut entry in &mut self.mapxy {
            if entry.x_pos == x && entry.y_pos == y {
                entry.air = air;
            }
        }
    }

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
            true
        }
    }

    pub fn show(&self) -> Vec<String> {
        let mut result = Vec::new();
        for y in 0..self.max_distance_graphic {
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
        // Positive x
        for x in 0..self.max_distance_graphic {
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
        // Negative x
        for x in 0..self.max_distance_graphic {
            for y in 0..self.max_distance_graphic {
                let new_sw = Coord {
                    x: (-1 - side * x), 
                    y: (side * y), 
                }; 
                let new_se = Coord {
                    x: (-1 - side * x), 
                    y: (side * y) + side -1,
                };
                let new_nw = Coord {
                    x: -1 - ((side * x) + side -1), 
                    y: side * y,
                };
                let new_ne = Coord {
                    x: -1 - ((side * x) + side -1), 
                    y: (side * y) + side -1
                };
                let sq = Area {
                    x_pos: x,
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

    pub fn show_map_as_str(&self) -> Vec<String> {
        let mut result = [].to_vec();
        for line in self.mapxy.to_owned() {
            let mut linestring = [format!("{:?}", line).to_owned()].to_vec();
            result.append(&mut linestring);
        }
        result
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustbrain(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Coord>()?;
    m.add_class::<Area>()?;
    m.add_class::<Datapoint>()?;
    m.add_class::<Dataset>()?;
    Ok(())
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
    assert_eq!(result.x, expected.x);
    assert_eq!(result.y, expected.y);
    let expectedshow = [
        "....", 
        "....", 
    ];
    assert_eq!(test_something.show(), expectedshow);
}
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
/*
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
*/
