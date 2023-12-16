use configs::ConfigTrait;

use crate::SearchEngineConfigs;

pub fn is_manage_searchable(manage_id: &String) -> bool {
    !SearchEngineConfigs::get()
        .unsearchable_manages
        .contains(manage_id)
}
