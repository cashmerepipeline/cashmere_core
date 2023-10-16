// 需要手工转
Map<String, String> nameMapFromMap(Map<String, dynamic> inMap) {
  final nameMap = <String, String>{};
  inMap.forEach((key, value) {
    nameMap[key] = value.toString();
  });
  return nameMap;
}
