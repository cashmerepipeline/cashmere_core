use cash_result::OperationResult;
use configs::ConfigTrait;
use dependencies_sync::{
    log,
    rust_i18n::{self, t},
    tokio::time,
};

use crate::{
    get_tantivy_writer,
    search_engine_runtime::{get_search_engine_runtime, is_search_engine_runtime_shutdown},
    SearchEngineConfigs,
};

pub fn spaw_commit_thread() -> Result<(), OperationResult> {
    let writer_arc = get_tantivy_writer();

    let run_time = get_search_engine_runtime();
    let commit_interval = SearchEngineConfigs::get().writer_commit_interval;

    run_time.spawn(async move {
        loop {
            time::sleep(time::Duration::from_secs(commit_interval)).await;
            let mut writer = writer_arc.write();

            match writer.commit() {
                Ok(_stamp) => {
                    log::info!(
                        "\t{}", //-{}: \t{}", // \t{}",
                        t!("索引提交成功"),
                    );
                }
                Err(e) => {
                    log::error!("{}: {:?}", t!("索引提交失败"), e);
                }
            };

            if is_search_engine_runtime_shutdown() {
                break;
            }
        }
    });
    Ok(())
}
