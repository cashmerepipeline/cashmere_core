const fs = require("fs");

const area_codes = require("./area_codes.json").sort((a, b) => a.code - b.code);

const codes_map = new Map();

area_codes.forEach((area) => {
  let item = codes_map.get(area.code);
  if (item) {
    item[2002].push({ "zh": area.name, "en": area.enName });
    codes_map.set(area.code, item);
  } else {
    codes_map.set(area.code, {
      1000: area.code,
      1002: "8610000000000",
      1001: area.code,
      2001: parseInt(area.code),
      2002: [{ "zh": area.name, "en": area.enName }],
      1006: "8610000000000",
      1007: ["1000000",],
    });
  }
});

const write_stream = fs.createWriteStream("phone_codes.txt");
codes_map.forEach((v, key) => {
  console.log(v);
  const areasStr = JSON.stringify(v[2002]);
  write_stream.write(`${key} = { 1000 = "${v[1000]}", 1002 = "${v[1002]}", 1001 = "${v[1001]}", 2001 = ${parseInt(v[2001])}, 1006 = "8610000000000", 1007 = ["1000000",], 2002 = ${areasStr} }`.replaceAll(':', '='));
  write_stream.write("\n");
})

write_stream.end();

write_stream.on("finish", () => {
  console.log("生成手机区号编码完成。");
}
);
