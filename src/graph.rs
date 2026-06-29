use adj_graph::AdjGraph;
use mat_graph::MatGraph;

use std::{collections::HashMap, num::NonZeroU8};
use anyhow::anyhow;

mod adj_graph;
mod mat_graph;

type EdgeLabel = NonZeroU8;

pub trait Graph {
    fn contains_odd_dominated_loop(self) -> bool;
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

    pub fn build_adj(self) -> AdjGraph {
        let mut edges = vec![Vec::new(); self.vertex_count];
        for ((v1, v2), label) in self.edges.iter() {
            edges[*v1].push((*v2, *label));
        }
        AdjGraph {
            vertex_count: self.vertex_count,
            edges
        }
    }

    pub fn build_mat(&self) -> MatGraph {
        let mut edges = vec![vec![None; self.vertex_count]; self.vertex_count];
        for ((v1, v2), label) in self.edges.iter() {
            edges[*v1][*v2] = Some(*label);
        }
        MatGraph {
            vertex_count: self.vertex_count,
            edges
        }
    }
}