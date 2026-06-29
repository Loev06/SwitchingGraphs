use super::{EdgeLabel, Graph};

use std::fmt;

pub struct MatGraph {
    pub vertex_count: usize,
    pub edges: Vec<Vec<Option<EdgeLabel>>>,
    pub edge_steps: usize,
}

impl Graph for MatGraph {
    // probably n^3 log n
    fn contains_odd_dominated_loop(mut self) -> bool {
        if self.edge_steps > self.vertex_count {
            return false;
        }
        self.edge_steps *= 2;
        for i in 0..self.vertex_count {
            for j in i..self.vertex_count {
                let e1 = self.update_edge(i, j);
                let e2 = if i == j {
                    e1
                } else {
                    self.update_edge(j, i)
                };
                if let (Some(l1), Some(l2)) = (e1, e2)
                    && (l1.get() % 2 == 1 && l1 >= l2
                     || l2.get() % 2 == 1 && l2 >= l1)
                {
                    return true;
                }
            }
        }
        self.contains_odd_dominated_loop()
    }
}

impl MatGraph {
    fn update_edge(&mut self, v1: usize, v2: usize) -> Option<EdgeLabel> {
        let mut value = self.edges[v1][v2];
        for i in 0..self.vertex_count {
            if let (Some(l1), Some(l2)) = (self.edges[v1][i], self.edges[i][v2]) {
                let branch = Some(l1.max(l2));
                value = self.max_odd(value, branch);
            }
        }
        self.edges[v1][v2] = value;
        value
    }
}

impl fmt::Display for MatGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "After {} steps:", self.edge_steps)?;
        for row in self.edges.iter() {
            for edge in row {
                if let Some(l) = edge {
                    write!(f, "{} ", l)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
