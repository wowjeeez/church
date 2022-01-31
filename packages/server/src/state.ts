import {getCallSites} from "./utils";

const serverState: State = {}
const listeners: {ref: Function, deps: string[]}[] = []
type State = Record<string, any>
import cwd, {isDev} from "./index"
import log from "./logger"
function callFactory(args: unknown[], atKey: string) {
    return (ref: any) => {
        ref[atKey](...args)
    }
}

function dispatchStateUpdate<T extends string>(key: T, val: any) {
    listeners.filter(listener => listener.deps.includes(key) || !listener.deps.length).map(callFactory([serverState, key, val], "ref"))
}

function updateServerState<T extends State, K extends keyof T>(key: K, value: T[K]) {
    (serverState as any)[key] = value
    dispatchStateUpdate(key as any, value)
}

export function useStateEffect<T extends State, K extends keyof T>(handler: (state: T, updatedKey: K, newVal: T[K]) => void, onKeys: (keyof T | string)[] = []) {
    // @church(cfg(!dev)): ignore-start
    if (isDev) {
        const handlerSrc = handler.toString()
        const isCyclic = handlerSrc.includes("useServerState") || handlerSrc.includes("updateServerState")
        if (isCyclic) {
            const stack = getCallSites(new Error("foo"))[0]
            log({location: "internal/state", level: "error", async: true}, `WARNING: In ${stack.getFileName().replace(cwd, "").replace("\\", "/")}:${stack.getLineNumber()} You are calling \`useServerState\` or \`updateServerState\` inside a useStateEffect, this could lead to bad things like stack overflows or crashes.`)
        }
    }
    // @church(cfg(!dev)): ignore-end
listeners.push({ref: handler, deps: onKeys as any})
}

export function useServerState<T extends State, F extends keyof T>(): [<K extends keyof T>(key?: K) => T, (key: F, value: T[F]) => void] {
    return [<K extends keyof T>(key?: K): T => key ? serverState[key as any] : serverState as T, updateServerState]
}

