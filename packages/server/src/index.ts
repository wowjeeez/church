import {test} from "./test";
const cwd = process.cwd()
export default cwd
export const isDev = !!process.env.DEV
test()