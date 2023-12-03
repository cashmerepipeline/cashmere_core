import 'dart:collection';
import 'dart:typed_data';

import 'package:bson/bson.dart';

Map<String, String> nameMapFromBytes(List<int> message) {
  final Map<String, String> nameMap = BsonCodec.deserialize(BsonBinary.from(message)).map((k, v) {
    return MapEntry<String, String>(k, v.toString());
  });

  return nameMap;
}
