use super::{EdgeLabel, Graph};

use std::fmt;

pub struct MatGraph {
    pub vertex_count: usize,
    pub edges: Vec<Vec<Option<EdgeLabel>>>
}

impl Graph for MatGraph {
    fn contains_odd_dominated_loop(self) -> bool {
        false
    }
}

impl fmt::Display for MatGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.edges.iter() {
            for edge in row {
                if let Some(l) = edge {
                    write!(f, "{}", l)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
