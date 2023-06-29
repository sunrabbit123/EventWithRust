import test from "ava";

import { EventEmitter } from "../index.js";

test("sum from native", (t) => {
  const ee = new EventEmitter();
  ee.on("foo", () => console.log("bar"));
  // t.is(sum(1, 2), 3);
});
