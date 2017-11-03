var addon = require('../native');

console.log(new Uint8Array(addon.hello()));
