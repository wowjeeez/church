/* Based on https://github.com/watson/error-callsites */

const callsitesSym: symbol = Symbol("callsites");
export { callsitesSym };

// Lifted from Node.js 0.10.40:
// https://github.com/nodejs/node/blob/0439a28d519fb6efe228074b0588a59452fc1677/deps/v8/src/messages.js#L1053-L1080
export function FormatStackTrace(
    error: Error,
    frames: NodeJS.CallSite[]
): string {
    const lines: string[] = [];
    try {
        lines.push(error.toString());
    } catch (e) {
        lines.push("<error: " + e + ">");
    }
    for (let i: number = 0; i < frames.length; i++) {
        const frame: NodeJS.CallSite = frames[i];
        let line: string;
        try {
            line = frame.toString();
        } catch (e) {
            line = "<error: " + e + ">";
        }
        lines.push("    at " + line);
    }
    return lines.join("\n");
}

const fallback: (err: Error, stackTraces: NodeJS.CallSite[]) => string =
    Error.prepareStackTrace || FormatStackTrace;

let lastPrepareStackTrace: (
    err: Error,
    stackTraces: NodeJS.CallSite[]
) => unknown = fallback;

function prepareStackTrace(
    err: Error,
    callsites: NodeJS.CallSite[]
): NodeJS.CallSite[] | unknown {
    // If the symbol has already been set it must mean that someone else has also
    // overwritten `Error.prepareStackTrace` and retains a reference to this
    // function that it's calling every time it's own `prepareStackTrace`
    // function is being called. This would create an infinite loop if not
    // handled.
    if (Object.prototype.hasOwnProperty.call(err, callsitesSym)) {
        return fallback(err, callsites);
    }

    Object.defineProperty(err, callsitesSym, {
        enumerable: false,
        configurable: true,
        writable: false,
        value: callsites,
    });

    return (
        (lastPrepareStackTrace && lastPrepareStackTrace(err, callsites)) ??
        err.toString()
    );
}

Object.defineProperty(Error, "prepareStackTrace", {
    configurable: true,
    enumerable: true,
    get: function () {
        return prepareStackTrace;
    },
    set: function (fn?: (err: Error, stackTraces: NodeJS.CallSite[]) => string) {
        // Don't set `lastPrepareStackTrace` to ourselves. If we did, we'd end up
        // throwing a RangeError (Maximum call stack size exceeded).
        lastPrepareStackTrace =
            fn === prepareStackTrace
                ? fallback
                : (fn as (err: Error, stackTraces: NodeJS.CallSite[]) => string);
    },
});

export function getCallSites(err: Error): NodeJS.CallSite[] {
    // @ts-ignore
    return (err.stack ? err[callsitesSym] : err[callsitesSym]) || [];
}

export function newRef<T extends any>(obj: T): T {
    let newObj;

    // To copy something that is not an object, just return it:
    if(typeof obj !== 'object' && typeof obj !== 'function' || obj == null)
        return obj;

    if(typeof obj === 'object') {
        newObj = {}
        const proto = Object.getPrototypeOf(obj)
        Object.setPrototypeOf(newObj, proto)
    } else {
        let aux
        newObj = eval('aux='+obj.toString()) //huge security vulnerability, should be fixed
        newObj.prototype = (obj as any).prototype
    }

    for(const i in obj) {
        if(obj.hasOwnProperty(i)) {

            if(typeof obj[i] !== 'object')
                newObj[i] = obj[i]
            else
                newObj[i] = newRef(obj[i])
        }
    }

    return newObj;
}