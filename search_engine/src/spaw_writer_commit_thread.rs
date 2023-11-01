use cash_result::OperationResult;
use dependencies_sync::{
    log,
    rust_i18n::{self, t},
    tokio::{spawn, time},
};

use crate::{manage_index_writer_map::get_manage_index_writer_map, search_engine_runtime::get_search_engine_runtime};

pub fn spaw_writer_commit_thread() -> Result<(), OperationResult> {
    let writer_map_arc = get_manage_index_writer_map();
    let writer_map = writer_map_arc.read();
    
    let run_time = get_search_engine_runtime();

    for (manage_id, writer_arc) in writer_map.iter() {
        let writer_arc = writer_arc.clone();
        let manage_id = *manage_id;
        run_time.spawn(async move {
            loop {
                time::sleep(time::Duration::from_secs(15)).await;
                let mut writer = writer_arc.write();

                match writer.commit() {
                    Ok(stamp) => {
                        log::info!(
                            "\t{}-{}: \t{}", // \t{}",
                            manage_id,
                            t!("索引提交成功"),
                            stamp,
                        );
                    }
                    Err(e) => {
                        log::error!("{}: {:?}", t!("索引提交失败"), e);
                    }
                };
            }
        });
    }
    Ok(())
}
