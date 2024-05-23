#[macro_export]
macro_rules! declare_get_manager {
    ($manager_type:ty, $manager:ident, $inner:expr) => {
        // 取得管理器
        pub async fn get_manager() -> &'static Manager {
            $manager.get_or_init(|| {
                Manager{ inner: $inner }
            })
        }
    };
}