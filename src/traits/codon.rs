pub trait Codon<B> {
    fn from_triplet(triplet: (B, B, B)) -> Self;
    fn anticodon(&self) -> Self;
}
