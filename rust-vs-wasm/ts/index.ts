import { greet, run } from "../pkg";

global.alert = console.log;

greet("wasm");
console.log(
  run([
    { x: 21, y: 35 },
    { x: 32, y: 74 },
  ])
);
