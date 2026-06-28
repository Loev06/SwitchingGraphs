use std::{collections::HashMap, fmt, num::NonZeroU8};
use anyhow::anyhow;

type EdgeLabel = NonZeroU8;

pub struct Graph {
    vertex_count: usize,
    edges: Vec<Vec<Option<EdgeLabel>>>
}

impl Graph {
    pub fn contains_odd_dominated_loop(self) -> bool {
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

    fn dfs(&self, cur: usize, target: usize, visited: &mut Vec<bool>) -> Option<EdgeLabel> {
        visited[cur] = true;
        let mut value = 0;
        for (next, &edge) in self.edges[cur].iter().enumerate() {
            if let Some(label) = edge {
                if next == target {
                    return Some(label);
                }
                if !visited[next]
                    && let Some(dfs) = self.dfs(next, target, visited)
                {
                    let branch = dfs.max(label).get();
                    value = self.max_odd(value, branch)
                }
            }
        }
        EdgeLabel::new(value)
    }

    fn max_odd(&self, a: u8, b: u8) -> u8 {
        match (a % 2 == 1, b % 2 == 1) {
            (true, true) | (false, false) => a.max(b),
            (true, false) => a,
            (false, true) => b,
        }
    }
}

impl fmt::Display for Graph {
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

pub struct GraphBuilder {
    vertex_count: usize,
    edges: HashMap<(usize, usize), EdgeLabel>
}

impl GraphBuilder {
    pub fn new(vertex_count: usize) -> Self {
        GraphBuilder {
            vertex_count,
            edges: HashMap::new()
        }
    }

    pub fn add_edge(mut self, v1: usize, v2: usize, label: u8) -> anyhow::Result<Self> {
        self.edges.insert((v1, v2), EdgeLabel::new(label).ok_or(anyhow!("Edge label must be non-zero"))?);
        Ok(self)
    }

    pub fn build(self) -> Graph {
        let mut edges = vec![vec![None; self.vertex_count]; self.vertex_count];
        for ((v1, v2), label) in self.edges {
            edges[v1][v2] = Some(label);
        }
        Graph {
            vertex_count: self.vertex_count,
            edges
        }
    }
}