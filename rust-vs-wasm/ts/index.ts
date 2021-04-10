import { greet, run } from "../pkg";
import { points } from "./tsp-def";

global.alert = console.log;

greet("wasm");
const result = run(points);
console.log(JSON.stringify(result, null, 2));
