use ndarray::{Array2, Array1};
use rand::Rng;
use pyo3::prelude::*;

#[pyclass]
pub struct QLearning {
    q_table: Array2<f64>,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64,
}

#[pymethods]
impl QLearning {
    #[new]
    fn new(state_size: usize, action_size: usize, learning_rate: f64, discount_factor: f64, epsilon: f64) -> Self {
        Self {
            q_table: Array2::zeros((state_size, action_size)),
            learning_rate,
            discount_factor,
            epsilon,
        }
    }

    fn update(&mut self, state: usize, action: usize, reward: f64, next_state: usize) {
        let max_next_q = self.q_table.row(next_state).iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let current_q = self.q_table[[state, action]];
        
        let new_q = current_q + self.learning_rate * (reward + self.discount_factor * max_next_q - current_q);
        self.q_table[[state, action]] = new_q;
    }

    fn choose_action(&self, state: usize) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.epsilon {
            rng.gen_range(0..self.q_table.ncols())
        } else {
            self.q_table.row(state).iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .unwrap_or(0)
        }
    }

    fn get_q_table(&self) -> Vec<Vec<f64>> {
        self.q_table.outer_iter().map(|row| row.to_vec()).collect()
    }
}
