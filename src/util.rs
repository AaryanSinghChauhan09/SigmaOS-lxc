//! lxc model — Linux Containers.

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub state: String,
    pub image: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub containers: Vec<Container>,
}

impl Default for Model {
    fn default() -> Self { Self { containers: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str, image: &str) -> bool {
        self.containers.push(Container { name: name.into(), state: "stopped".into(), image: image.into() });
        true
    }

    pub fn start(&mut self, name: &str) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.name == name) { c.state = "running".into(); true } else { false }
    }

    pub fn list(&self) -> &[Container] { &self.containers }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_start() {
        let mut l = Model::new();
        assert!(l.create("ct1", "ubuntu"));
        assert!(l.start("ct1"));
        assert_eq!(l.list()[0].state, "running");
    }
}
