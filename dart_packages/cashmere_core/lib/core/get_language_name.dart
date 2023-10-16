String? getLanguageName(Map<String, String> languageMap, String languageCode) {
  return languageMap[languageCode];
}

String? getFirstLanguageName(Map<String, String> languageMap) {
  return languageMap[languageMap.keys.first];
}
