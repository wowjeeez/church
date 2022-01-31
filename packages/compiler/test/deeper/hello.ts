import {test} from "./f2";
debug("HELLO", ...spread)
console.log("test", test)
// @church(cfg(DEV == 10)): ignore-start
console.log("test")
// @church(cfg(!DEV)): ignore-end

// @church(): echo hello