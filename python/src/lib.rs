use std::collections::HashMap;

use numpy::{Ix1, Ix2, PyArrayLike1};
use numpy::{PyArrayLike, PyArrayLikeDyn};
// use crfs_rs::{Attribute, Model};
use pyo3::prelude::*;

use robotsim::robot::Robot;

use eyre::Result;

#[feature(visualiser)]
mod visualiser;

#[pyclass(module = "robotsim", name = "Robot")]
// #[self_referencing]
struct PyRobot {
    #[pyo3(get, set)]
    pub data: Vec<u8>,
    // #[borrows(data)]
    // #[covariant]
    robot: Robot,
}

#[pymethods]
impl PyRobot {
    #[new]
    fn py_new(path: &str) -> PyResult<Self> {
        Ok(PyRobot {
            data: vec![5, 9],
            robot: Robot::from_file(path)?,
        })
    }

    #[getter]
    fn name(&self) -> &str {
        self.robot.name()
    }

    #[getter]
    fn joints(&self) -> Vec<f32> {
        // self.robot.name()
        self.robot.robot_chain.joint_positions()
    }

    #[getter]
    fn joint_limits_by_order(&self) -> Option<Vec<(f32, f32)>> {
        self.robot
            .robot_chain
            .iter_joints()
            .map(|joint| {
                joint
                    .limits
                    .map(|limit| (limit.min, limit.max))
            })
            .collect()
    }

    #[getter]
    fn joint_limits(&self) -> Option<HashMap<String, (f32, f32)>> {
        self.robot
            .robot_chain
            .iter_joints()
            .map(|joint| {
                joint
                    .limits
                    .map(|limit| (joint.name.clone(), (limit.min, limit.max)))
            })
            .collect()
    }

    #[getter]
    fn joint_link_map(&self) -> HashMap<String, String> {
        // self.robot.name()
        self.robot.joint_link_map.clone()
    }

    #[getter]
    fn joint_names(&self) -> Vec<String> {
        // self.robot.name()
        self.robot
            .robot_chain
            .iter_joints()
            .map(|joint| joint.name.clone())
            .collect()
    }

    #[getter]
    fn link_names(&self) -> Vec<String> {
        // self.robot.name()
        self.robot
            .robot_chain
            .iter_links()
            .map(|link| link.name.clone())
            .collect()
    }

    // #[pyfunction]
    // fn sum_up<'py>(py: Python<'py>, array: PyArrayLike2<'py, f32, AllowTypeChange>) -> f32 {

    fn set_joints(&mut self, array: PyArrayLike1<f32, AllowTypeChange>) -> Result<()> {
        Ok(self.robot.robot_chain.set_joint_positions(array.as_slice()?)?)
    }

    fn has_collision(&mut self, array: PyArrayLike2<f32, AllowTypeChange>) -> Result<Vec<bool>> {
        let a: Result<Vec<_>> = array
            .as_array()
            .rows()
            .into_iter()
            .map(|row| {
                // dbg!(a.ok_or_eyre("Failed to get slice (array is not contiguous?)")).unwrap();
                // println!("{:?}", row);
                match self.robot.has_collision(
                    row.as_slice()
                        .ok_or_eyre("Failed to get slice (array is not contiguous?)")?,
                ) {
                    Ok(result) => match dbg!(&result) {
                        robotsim::robot::CollisionResult::Free => Ok(false),
                        robotsim::robot::CollisionResult::Collision => Ok(true),
                        robotsim::robot::CollisionResult::OutOfJointLimit => Ok(true),
                    },
                    Err(e) => {
                        println!("{}", e);
                        dbg!(e.chain().collect::<Vec<_>>());
                        Ok(false)
                    }
                }
            })
            .collect();

        a
    }

    fn __repr__(&self) -> String {
        format!("<Robot '{}'>", self.name())
    }
}

use eyre::OptionExt;
use numpy::{get_array_module, AllowTypeChange, PyArrayLike2};

#[pyfunction]
fn sum_up<'py>(py: Python<'py>, array: PyArrayLike2<'py, f32, AllowTypeChange>) -> f32 {
    array.as_array().rows().into_iter().for_each(|row| {
        let a = row.as_slice();

        dbg!(a.ok_or_eyre("Failed to get slice (array is not contiguous?)")).unwrap();
        println!("{:?}", row);
    });

    dbg!(array.as_slice().unwrap());
    array.as_array().sum()
}

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pymodule]
#[pyo3(name = "robotsim")]
mod py_robotsim {
    use super::*;

    #[pymodule_export]
    use super::double; // Exports the double function as part of the module

    #[pymodule_export]
    use super::sum_up; // Exports the double function as part of the module

    #[pymodule_export]
    use super::PyRobot;

    #[pyfunction] // This will be part of the module
    fn triple(x: usize) -> usize {
        x * 3
    }

    #[pymodule_export]
    use visualiser::PyVisualiser;

    // #[pyfunction] // This will be part of the module
    // fn start_visualiser() {
    //     visualiser::start_visualiser();
    // }

    #[pyclass] // This will be part of the module
    struct Unit;

    #[pymodule]
    mod submodule {
        // This is a submodule
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Arbitrary code to run at the module initialization
        m.add("double2", m.getattr("double")?)
    }
}
