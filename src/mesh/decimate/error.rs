#[derive(Debug, Copy, Clone)]
struct Error(f64);

impl PartialOrd for Error {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Error {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 > other.0 {
            return Ordering::Greater;
        } else if self.0 < other.0 {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl Eq for Error {

}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}