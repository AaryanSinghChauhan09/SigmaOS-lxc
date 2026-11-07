//! lxc model — Linux Containers.

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub status: String,
    pub running: bool,
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

    pub fn create(&mut self, name: &str, image: &str) -> usize {
        let id = self.containers.len();
        self.containers.push(Container { name: name.into(), image: image.into(), status: "created".into(), running: false });
        id
    }

    pub fn start(&mut self, name: &str) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.name == name) {
            c.running = true;
            c.status = "running".into();
            true
        } else { false }
    }

    pub fn stop(&mut self, name: &str) -> bool {
        if let Some(c) = self.containers.iter_mut().find(|c| c.name == name) {
            c.running = false;
            c.status = "stopped".into();
            true
        } else { false }
    }

    pub fn list(&self) -> &[Container] { &self.containers }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_start_stop() {
        let mut l = Model::new();
        l.create("ubuntu", "ubuntu:22.04");
        assert!(l.start("ubuntu"));
        assert!(l.stop("ubuntu"));
    }
}
