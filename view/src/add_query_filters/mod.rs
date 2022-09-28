use bson::{self, doc, Document};
use linked_hash_map::LinkedHashMap;
use manage_define::general_field_ids::{GROUPS_FIELD_ID, OWNER_FIELD_ID};

use crate::view_rule::ViewRule;
use crate::view_rules_map::get_view_rules_map;
use crate::{FilterRule, ReadRule};

mod get_readable_groups;
mod get_read_nolimit_groups;
mod is_only_owner;
mod add_query_filters;
mod check_group_read_nolimit;
mod check_group_read_group;
mod check_group_read_only_owner;

pub use add_query_filters::add_query_filters;

pub(crate) use get_read_nolimit_groups::get_read_nolimit_groups;
pub(crate) use get_readable_groups::get_readable_groups;
pub(crate) use is_only_owner::is_only_owner;