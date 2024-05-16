pub trait States: 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug + Default {
    type Iter: Iterator<Item = Self>;

    //Returns an iterator over all the state variants
    fn variants() -> Self::Iter;
}