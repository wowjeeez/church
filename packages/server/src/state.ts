import {getCallSites} from "./utils";
import cwd, {createFiber, isDev, useFiber} from "./index"
import log from "./logger"
import {Fiber, StateUpdate} from "./fiber";

const svState = createFiber<State>("CH_SVSTATE", false, false)
type State = { [key: string]: any }


export function useServerStateEffect<T extends Record<string, any>, Keys extends ReadonlyArray<keyof T>>(handler: StateUpdate<T, Keys>, keys?: Keys) {
    if (handler.toString().includes("useServerState")) {
        const stack = getCallSites(new Error("foo"))[0]
        log({location: "internal/state", level: "error", async: true}, `WARNING: In ${stack.getFileName().replace(cwd, "").replace("\\", "/")}:${stack.getLineNumber()} You are calling \`useServerState\` inside a useStateEffect, this could lead to bad things like stack overflows or crashes.`)
    }
    svState.listen(<any>handler, <readonly string[]>keys)
}

export function useServerState<T extends Record<string, any>>(): Fiber<T> {
    return useFiber<T>("CH_SVSTATE")
}