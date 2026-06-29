use super::{EdgeLabel, Graph};

use std::fmt;

pub struct AdjGraph {
    pub vertex_count: usize,
    pub edges: Vec<Vec<(usize, EdgeLabel)>>
}

impl Graph for AdjGraph {
    fn contains_odd_dominated_loop(self) -> bool {
        for v in 0..self.vertex_count {
            let mut visited = vec![false; self.vertex_count];
            if let Some(max_odd) = self.dfs(v, v, &mut visited)
                && max_odd.get() % 2 == 1
            {
                return true;
            }
        }
        false
    }
}

impl AdjGraph {
    fn dfs(&self, cur: usize, target: usize, visited: &mut Vec<bool>) -> Option<EdgeLabel> {
        visited[cur] = true;
        let mut value = None;
        for (next, label) in self.edges[cur].iter() {
            if *next == target {
                return Some(*label);
            }
            if !visited[*next]
            {
                let dfs = self.dfs(*next, target, visited);
                let branch = dfs.max(Some(*label));
                value = self.max_odd(value, branch)
            }
        }
        value
    }
}

impl fmt::Display for AdjGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v1 in 0..self.vertex_count {
            for (v2, label) in &self.edges[v1] {
                writeln!(f, "{}-{}: {}", v1, v2, label)?;
            }
        }
        Ok(())
    }
}