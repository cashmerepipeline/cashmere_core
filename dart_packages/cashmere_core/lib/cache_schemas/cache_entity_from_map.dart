
import 'package:bson/bson.dart';
import 'package:objectbox/objectbox.dart';

/// 从map新建约束函数
typedef CacheEntityFromMap<Ct> = Ct Function(Map<String, dynamic> map);

/// 取得cache中最近修改的时间戳
typedef GetLastModifiedTimestamp<Ct> = Future<Timestamp> Function(Box<Ct> box);