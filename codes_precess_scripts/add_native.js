const fs = require("fs");

const langs = require("./languages.json");
const codes = require("./language_codes.json");

native_codes = codes.map(code => {
    lang = langs[code.code.split('-')[0]];
    if (lang !== undefined) {
        code.native = lang.native;
    }
    return code;
})

fs.writeFileSync("language_codes_native.json", JSON.stringify(native_codes, null, 2));