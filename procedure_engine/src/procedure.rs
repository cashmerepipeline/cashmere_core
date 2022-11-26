/// 过程
struct Procedure {
    // id
    work_id: String,
    phase: Option<Phase>,
    // 不同阶段对应不同工作图
    work_graphs: Option<LinkedHashMap<String, WorkGraph>>,
}
