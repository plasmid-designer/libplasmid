pub trait NucleoBase {
    fn bonding_partner(&self) -> Self;
    fn base_pair(&self) -> (Self, Self)
    where
        Self: Sized + Copy,
    {
        (*self, self.bonding_partner())
    }
}
