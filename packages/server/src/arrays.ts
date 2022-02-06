import {klona} from "klona/full";

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
            },
            /**
             * Groups an array by the specified keys. Every key has to be equal in order to be put into a group, if even 1 value is different, a new group will be created.
             * @param keys: string[]
             * @param functionArgMap: Record<string, any[]>, Arguments to pass to the functions in keys
             * @return Object keyed with JSON serialized values of the key results
             */
            groupBy: <K extends ReadonlyArray<keyof T[number]>, FnArgs>(keys: K, functionArgMap: Partial<Record<K[number], any[]>> = {}) => {
                const map = new Map<string, any>()
                for (const v of arr) {
                    const results = []
                        for (const key of keys) {
                            if (typeof v[key] == "function") {
                                const args = functionArgMap[key] || []
                                // @ts-ignore
                                const ret = v[key](...args)
                                results.push(ret)
                            }  else {
                                results.push(v[key])
                            }
                        }
                        const jsonStr = JSON.stringify(results)
                        const curr: any[] = map.get(jsonStr) || []
                        map.set(jsonStr, curr)
                }
                const returnObj: Record<string, T[number][]> = {}
                map.forEach((val, key) => {
                    returnObj[key] = val
                })
                return returnObj
            },
            /**
             * Returns a reference to the original array
             */
            inner: () => arr,
            /**
             * Partitions an array
             * @param size The partition size
             * @param dropRemaining Drop the remaining end of the array
             */
            partition: (size: number = 10, dropRemaining = false) => {
                const result: any[] = [];

                for (let i = 0; i < arr.length; i++) {
                    if (i % size === 0) result.push([]);
                    result[result.length - 1].push(arr[i]);
                }
                if (dropRemaining) {
                    if (arr.length % size === 0) {
                        return useArray(result)
                    } else {
                        const highestIndex = (Math.min(result.length / size) * size) - 1
                        return useArray(result.slice(0, highestIndex))
                    }
                } else {
                    return useArray(result);
                }
            }
        }
}



