extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(rust_brain_libs, |py, m| {
    m.add(py, "__doc__", "This module implements additional libraries in Rust to add on ROS2's Python nodes.")?;
            m.add(py, "return_sum", py_fn!(py, return_sum(i: i32, j: i32)))?;
    Ok(())
});

fn return_sum(_py: Python, i: i32, j: i32) -> PyResult<i32> {
    Ok(i + j)
}

#[derive(Debug, Clone)]
struct Dataset {
    set: Vec<Datapoint>,
}

#[derive(Debug, Clone)]
struct Datapoint {
    x: i32,
    y: i32,
    solid: bool,
}

trait Mapping {
    fn show(&self) -> Vec<String>;
}

impl Mapping for Dataset {
    fn show(&self) -> Vec<String> {
        let mut result = Vec::new();
        for x in 1..11 {
            let mut line = "".to_owned();
            for y in 1..11 {
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


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn set_several_and_show() {
        let mut dataset = Dataset {
            set: [].to_vec(),
        };
        let this_dp = Datapoint {
            x: 10,
            y: 5,
            solid: true,
        };
        dataset.set.push(this_dp);

        assert_eq!(2 + 2, 4);
        let mut test: Vec<String> = Vec::new();
        test.push("..........".to_string());  
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("..........".to_string()); 
        test.push("....O.....".to_string()); 
        assert_eq!(test, dataset.show());
    }
}
