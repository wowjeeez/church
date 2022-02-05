import {klona} from "klona";

type Falsy = false | 0 | "" | null | undefined
type Truthy = number | true | any[] | {} | any

const falsyArr = [false, 0, "", null,undefined]


function objKeysEqual<T extends Record<any, any>>(o1: T, o2: Partial<T>): boolean {
    let ret = false
    for (const [k, v] of Object.entries(o2)) {
        if (typeof v == "object") {
            ret = objKeysEqual(o1[k], v)
        } else if (typeof v == "function") {
            ret = v.toString() == o1[k].toString()
        }
        else {
            ret = o1[k] == v
        }
    }
    return ret
}

/**
 * Advanced array methods
 * @param arr: any[]
 */
export default function useArray<T extends ReadonlyArray<unknown>>(arr: T) {
        return {
            /**
             * Will find an index of any object (can be partial)/value in the array
             * @param toFind
             */
            indexOf: (toFind: T[number] extends Record<any, any> ? Partial<T[number]> : T[number]): number => {
                const objMode = typeof  toFind === "object"
                for (const i in arr) {
                 const v = arr[i]
                    if (objMode ? objKeysEqual(v, toFind) : v === toFind) {
                        return i as unknown as number
                    }
                }
                return -1
            },
            /**
             * Async map over the array
             * @param fn
             */
            map: async <TT extends any>(fn: (v: T[number], k: number, arr: T) => Promise<TT> | TT): Promise<ReadonlyArray<TT>> => {
                const newVal = []
                for (const i in arr) {
                    const v = arr[i]
                    const mres = await fn(v, i as unknown as number, arr)
                    newVal.push(mres)
                }
                return newVal
            },
            /**
             * Async iterator over the array
             * @param fn
             */
            iter: async (fn: (v: T[number], k: number, arr: T) => Promise<void> | void) => {
                for (const i in arr) {
                    const v = arr[i]
                    await fn(v, i as unknown as number, arr)
                }
            },
            /**
             * Alias for iter()
             */
            forEach: this.iter,
            filter: async (fn: (v: T[number], k: number, arr: T) => Promise<Falsy | Truthy> | Falsy | Truthy) => {
                const res: T[number][] = []
                for (const i in arr) {
                    const v = arr[i]
                    const falsy = falsyArr.includes(await fn(v, i as unknown as number, arr))
                    if (!falsy) {
                        res.push(v)
                    }
                }
                return res
            },
            findIndex: async (fn: (v: T[number], k: number, arr: T) => Promise<Falsy | Truthy> | Falsy | Truthy) => {
                for (const i in arr) {
                    const v = arr[i]
                    const falsy = falsyArr.includes(await fn(v, i as unknown as number, arr))
                    if (!falsy) {
                        return i as unknown as number
                    }
                }
                return -1
            }
        }
}