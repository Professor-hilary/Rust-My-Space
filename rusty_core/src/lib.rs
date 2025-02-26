mod q_learning;

use pyo3::prelude::*;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[pymodule]
fn rusty_core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<q_learning::QLearning>()?;
    Ok(())
}
