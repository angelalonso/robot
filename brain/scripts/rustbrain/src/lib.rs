#![feature(macro_attributes_in_derive_output)]
use pyo3::prelude::*;


#[derive(Debug, Clone)]
#[pyclass]
pub struct Datapoint {
    #[pyo3(get, set)]
    pub x: i32,
    #[pyo3(get, set)]
    pub y: i32,
    #[pyo3(get, set)]
    pub solid: bool,
}
#[pymethods]
impl Datapoint {
    #[new]
    pub fn new(x: i32, y: i32, solid: bool) -> Datapoint {
        Datapoint { x, y, solid }
    }
}
#[derive(Debug, Clone)]
#[pyclass]
pub struct Dataset {
    #[pyo3(get, set)]
    pub set: Vec<Datapoint>,
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
            min_angle: min_angle,
            max_angle: max_angle,
            max_distance: max_distance,
            max_distance_graphic: max_distance_graphic,
        }
    }
    pub fn add(&mut self, dp: Datapoint) {
        self.set.append(&mut [dp].to_vec())
    }
    // TODO: given angle and distance,
    //         generate a set of points in straight line, all solid=false but the one at the given
    //         distance
    pub fn add_ping(&mut self, angle_raw: i32, _distance: i32) -> i32 {
        // transform angle to degrees (-90 to 90)
        let angle = ((angle_raw - self.min_angle) * 180 / (self.max_angle - self.min_angle)) - 90;
        // find quadrants on that angle
        // mark all quadrants as solid=False but the one on distance=distance
        angle
    }

    pub fn show(&self) -> Vec<String> {
        let mut result = Vec::new();
        for x in 1..11 {
            let mut line = "".to_owned();
            for y in 1..21 {
                let mut object = false;
                for dp in self.set.clone() {
                    if dp.x == x && dp.y == y && dp.solid {
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
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustbrain(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Datapoint>()?;
    m.add_class::<Dataset>()?;
    Ok(())
}
