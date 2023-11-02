pub trait ConfigTrait {
    fn name() -> &'static str;
    fn get() -> &'static Self;
}
