use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Graph<W> {
    graph: Vec<Vec<(u32, W)>>,
}

impl<W> Graph<W>{
    // An iterator over the edges of the graph
    fn edges(&self) -> impl Iterator<Item = (u32, u32, &W)> + '_
    {
        self.graph.iter().enumerate().flat_map(|(u, v)|{
            let x: u32 = u32::try_from(u).expect("value too larget for u32");
            v.iter().map(move |(y,w)| (x, *y, w))
        })
    } 
}

impl Graph<()> {
    pub fn new(g: Vec<Vec<(u32, ())>>) -> Self {
        Self { graph: g }
    }
}

impl Graph<u32> {
    pub fn new(g: Vec<Vec<(u32, u32)>>) -> Self {
        Self { graph: g }
    }
}

impl fmt::Display for Graph<()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.edges();
        for (x,y,_) in i {
            writeln!(f, "{x}->{y}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Graph<u32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.edges();
        for (x, y, w) in i {
            writeln!(f, "{x}-({w})->{y}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn stub() {
        assert!(true);
    }
}
