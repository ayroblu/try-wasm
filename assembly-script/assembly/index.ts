import { print } from "./external";

export function printAdd(a: i32, b: i32): void {
  print(a + b);
}

export function add(a: i32, b: i32): i32 {
  return a + b;
}
