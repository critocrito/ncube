/* eslint @typescript-eslint/no-var-requires: off */
const {addHook} = require("pirates");

require("jsdom-global")();

window.Date = Date;

// Ignore the CSS imports to make the tests work.
addHook((code) => code.replace(/require.*\.css/, "$`//"), {
  exts: [".tsx"],
});
