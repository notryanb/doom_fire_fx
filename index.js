import init, { wasm_main } from "./pkg/doom_fire_fx.js";

async function run() {
  await init();
  wasm_main();
}

run();