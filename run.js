
const tauri = require('./pkg');

//tauri.main();

//tauri.tauri(["build"]);

let res = tauri.print_env();
console.log(res);
