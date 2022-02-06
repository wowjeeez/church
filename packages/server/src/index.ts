import {test} from "./test";

const cwd = process.cwd()
export default cwd
export const isDev = !!process.env.DEV

export {useDumbEvent, useNetEvent} from "./events"
export {createFiber, useFiber} from "./fiber"
export {useServerState, useStateEffect} from "./state"
export {useVector3, useVector2, useVectorialDistance} from "./vectors"
declare function debug(...msg: unknown[]): void
declare function trace(...msg: unknown[]): void
declare function info(...msg: unknown[]): void
declare function silly(...msg: unknown[]): void
declare function error(...msg: unknown[]): void
//@church-dev 
test()