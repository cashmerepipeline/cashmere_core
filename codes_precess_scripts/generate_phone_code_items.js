const fs = require("fs");

const area_codes = require("./area_codes.json").sort((a, b) => a.code - b.code);

const codes_map = new Map();

area_codes.forEach((area) => {
  let item = codes_map.get(area.code);
  if (item) {
    item[1001].zh = item[1001].zh + "," + area.name;
    item[1001].en = item[1001].en + "," + area.enName;
    item[2002].push(area.shortName.toLowerCase());
    codes_map.set(area.code, item);
  } else {
    codes_map.set(area.code, {
      1000: area.code,
      1002: "8610000000000",
      1001: {"zh": area.name, "en": area.enName},
      2001: area.code,
      2002: [area.shortName.toLowerCase()],
      1006: "8610000000000",
      1007: ["admin",],
    });
  }
});

const write_stream = fs.createWriteStream("phone_codes.toml");
codes_map.forEach((v, key) => {
  const nameMap = v[1001];
  const areasStr = JSON.stringify(v[2002]);
  write_stream.write(`${key} = { 1000 = "${v[1000]}", 1002 = "${v[1002]}", 1001 = {zh = "${nameMap.zh}", en = "${nameMap.en}"}, 2001 = "${v[2001]}", 1006 = "8610000000000", 1007 = ["admin",], 2002 = ${areasStr} }`.replaceAll(':', '='));
  write_stream.write("\n");
})

write_stream.end();

write_stream.on("finish", () => {
  console.log("生成手机区号编码完成。");
}
);
