/// 取得服务器语言设置
pub fn get_language_code() -> &'static String {
    let configs = crate::configs::get_configs();
    &configs.server.language_code
}
