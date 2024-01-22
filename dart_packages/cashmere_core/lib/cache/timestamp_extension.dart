/// Timestamp extension methods.
import "package:bson/bson.dart";
import "package:isar/isar.dart";

extension TimestampExtension on Timestamp {
  List<byte> serialize() {
    return BsonCodec.serialize({"v": this}).byteList;
  }
  DateTime toDateTime(){
    final result = DateTime.fromMillisecondsSinceEpoch(seconds*1000);
    return result.add(Duration(microseconds: increment));

  }
}

extension TimestampExtensionDateTime on DateTime {
  Timestamp toTimestamp() {
    return Timestamp(millisecondsSinceEpoch ~/ 1000, microsecond);
  }
}

extension TimestampExtensionIsar on List<byte>{
  Timestamp deserializeTimestamp() {
    return BsonCodec.deserialize(BsonBinary.from(this))["v"] as Timestamp;
  }
}