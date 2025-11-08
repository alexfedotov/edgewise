use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Graph {
    graph: Vec<Vec<u32>>,
}

impl Graph {
    pub fn new(g: Vec<Vec<u32>>) -> Self {
        Self { graph: g }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.graph)
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn stub() {
        assert!(true);
    }
}
