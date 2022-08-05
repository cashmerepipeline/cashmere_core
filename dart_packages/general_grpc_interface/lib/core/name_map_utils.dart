import 'dart:collection';
import 'dart:typed_data';

import 'package:bson/bson.dart';

Map<String, String> nameMapFromBytes(List<int> message) {
  final bson = BSON();
  final Map<String, String> nameMap =
      bson.deserialize(BsonBinary.from(message)).map((k, v) {
    return MapEntry<String, String>(k, v.toString());
  });

  return nameMap;
}
