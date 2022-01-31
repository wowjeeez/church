import log from "./logger"
const fibers: Record<string, Fiber<any>> = {}


class _Fiber {
    private readonly store: Record<string, any> = {}
    private readonly listeners: {handler: Function, triggers: ReadonlyArray<string>}[] = []
    private readonly snapshots: Record<string, any>[] = []
    constructor(private readonly snapshot: boolean) {}
    public setVal(key: string, value: any) {
        this.store[key] = value
        if (this.snapshot) {
            this.snapshots.push(this.store)
        }
        this.getActivatedListeners(key).map(val => val.handler(key, val))
    }
    public getVal<T>(key: string): T {
        return this.store[key]
    }
    public pushListener(handler: Function, interests: ReadonlyArray<string> = []) {
        this.listeners.push({handler, triggers: interests})
    }
    private getActivatedListeners(key: string) {
        return this.listeners.filter(list => !list.triggers.length || list.triggers.includes(key))
    }
    public getSnapshot<T>(index: number = this.snapshots.length - 1): T {
        if (this.snapshot) {
            return this.snapshots[index] as T
        } else {
            log({level: "error", async: true, location: "internal/fiber"}, "Warning: Called getSnapshot on a Fiber that doesn't have snapshots enabled.")
            return undefined
        }
    }
}

interface Fiber<Core extends Record<string, any>> {
    write: <T extends keyof Core>(key: T, value: Core[T]) => void,
    get: <T extends keyof Core>(key: T) => Core[T],
    listen: <Keys extends ReadonlyArray<keyof Core>>(handler: (key: Keys[number], value: Core[Keys[number]]) => void, keys?: Keys) => void
    getSnapshot: (index?: number) => Core
}


function _createFiber<T extends any>(name: string, takeSnapshots: boolean, replicated: boolean): Fiber<T> {
if (fibers[name]) {
    log({level: "error", location: "internal/fiber", async: true}, `Warning! Duplicate Fiber names. This is a fatal error! (${name})`)
}
const fibr = new _Fiber(takeSnapshots)
return {
    get: <K extends keyof T>(key: K): T[K] => fibr.getVal<T[K]>(key as string),
    listen: (handler, keys ) => fibr.pushListener(handler, keys as ReadonlyArray<string> || []),
    write: (key, val) => fibr.setVal(key as string, val),
    getSnapshot: index => fibr.getSnapshot(index)
}
}
export function createFiber<T extends Record<string, any>>(name: string, takeSnapshots: boolean, replicated: boolean): Fiber<T> {
    const fbr = _createFiber<T>(name, takeSnapshots, replicated)
    fibers[name] = fbr
    return fbr
}


export function useFiber<T extends Record<string, any>>(name: string): Fiber<T> {
    return fibers[name]
}

