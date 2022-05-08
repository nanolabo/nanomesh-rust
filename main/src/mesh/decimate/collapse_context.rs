#[derive(Debug, Copy, Clone)]
struct CollapseContext {
    collapse_to: DVec3,
    error: f64, // TODO: f32 ?
    weight: f64, // TODO: f32 ?
}

impl Default for CollapseContext {
    fn default() -> Self {
        Self {
            collapse_to: DVec3::default(),
            error: 0.,
            weight: 0.,
        }
    }
}

impl PartialOrd for CollapseContext {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CollapseContext {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.error > other.error {
            return Ordering::Greater;
        } else if self.error < other.error {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl Eq for CollapseContext {

}

impl PartialEq for CollapseContext {
    fn eq(&self, other: &Self) -> bool {
        self.error == other.error
    }
}