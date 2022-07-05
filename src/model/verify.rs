pub trait Verify {
    fn verify(&self, client: &ureq::Agent) -> bool;
}
