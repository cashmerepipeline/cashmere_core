/*
Author: 闫刚 (yes7rose@sina.com)
data.rs (c) 2020
Desc: 数据
Created:  2020-11-25T02:58:16.832Z
Modified: !date!
*/

use bson::Bson;
use std::{any::Any, sync::Arc};
use futures::Stream;
use bytes::Bytes;

enum ManageDataType {
    File(String),
    Sequence(String),
    Set(Vec<String>),
    Folder(Vec<String>),
    Bson(Bson),
    Stream(Arc<dyn Stream<Item = Bytes>>)
}