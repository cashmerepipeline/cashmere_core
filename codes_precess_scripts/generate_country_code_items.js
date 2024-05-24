const fs = require("fs");

const countries = require("./countries.json");
const areas = require("./area_codes.json");

const codes_map = new Map();

for (const key of Object.keys(countries)) {
  const key_l = key.toLowerCase();
  const country_info = countries[key];
  const area = areas.find((area) => area.shortName === key);
  if (area === undefined) {
    console.log(`找不到${key}的中文名。`);
    continue;
  }

  const zh_name = area.name;

  codes_map.set(key, {
    1000: key_l,
    1002: "8610000000000",
    1001: { "en": country_info.name, "zh": zh_name },
    2001: country_info.native,
    2002: key_l,
    2003: country_info.phone[0],
    2004: country_info.languages,
    1006: "8610000000000",
    1007: ["admin",],
  });
}

const write_stream = fs.createWriteStream("countries_codes.toml");
codes_map.forEach((v, key) => {
  const name_map = v[1001];
  write_stream.write(`${key.toLowerCase()} = { 1000 = "${v[1000]}", 1002 = "${v[1002]}", 1001 = {zh = "${name_map.zh}", en="${name_map.en}"}, 1006 = "8610000000000", 1007 = ["admin",],2001 = "${v[2001]}", 2002 = "${v[2002]}", 2003=${v[2003]}, 2004=${JSON.stringify(v[2004])} }`.replaceAll(':', '='));
  write_stream.write("\n");
})

write_stream.end();

write_stream.on("finish", () => {
  console.log("生成国家编码完成。");
}
);
