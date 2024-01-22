import 'package:bson/bson.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:cashmere_core/cache/timestamp_extension.dart';

void main(){
  final t = Timestamp(99, 123);
  final st = BsonCodec.serialize({"v": t}).byteList;
  final et = EJsonCodec.deserialize(BsonBinary.from(st));
  debugPrint("$st");
  debugPrint("$et");

  final tt = t.toDateTime();
  debugPrint("$tt");

  final stt = tt.toTimestamp();
  debugPrint("$stt");

  final dt = BsonCodec.deserialize(BsonBinary.from(st))["v"] as Timestamp;
  debugPrint("$dt");

  test("测试反序列化", (){
    expect(dt.seconds, equals(99));
  });
}