const handlersWithState: Record<string, { registeredListenerRef?: Function, handlers: Function[], state: Record<string, any>, stateListeners: {onKeys: string[], handler: Function}[]}> = {}
const handlersWithoutState: Record<string, { registeredListenerRef?: Function, handlers: Function[] }> = {}

const init = (name: string, dumb: boolean) => {
    if (!dumb) {
        if (!handlersWithState[name]) {
            handlersWithState[name] = {handlers: [], state: {}, stateListeners: []}
        }
    } else {
        if (!handlersWithoutState[name]) {
            handlersWithoutState[name] = {handlers: []}
        }
    }
}

interface State {
    /**
     * Attach a value to the state at a key
     * @param key string
     * @param value any
     */
    attach: (key: string, value: any) => void,
    /**
     * Bind a listener to the state that will fire if the listened keys change or there is no key specified and the state is updated
     * @param handler
     * @param onKeys string[]
     */
    onChange: <T>(handler: (key: string, value: T) => void, onKeys?: string[]) => void
    /**
     * Gets a value from the state
     * @param key string
     */
    getVal: <T>(key: string) => T
    /**
     * Returns the current player
     */
    src: () => number

}

function unregisterEventHandler(name: string, idx: number, dumb: boolean) {
    if (!dumb) {
        delete handlersWithState[name].handlers[idx]
        handlersWithState[name].handlers = handlersWithState[name].handlers.filter(val => val)
    } else {
        delete handlersWithoutState[name].handlers[idx]
        handlersWithoutState[name].handlers = handlersWithoutState[name].handlers.filter(val => val)
    }
}

/**
 * Event handler with state
 */
export function useNetEvent<T extends unknown>(eventName: string, handler: (state: State, payload: T) => void) {
    init(eventName, false)
    const wrappedState = (src: number): State => ({
        attach: (key: string, value: any) => {
            handlersWithState[eventName].state[key] = value
            handlersWithState[eventName].stateListeners.filter(val => !val.onKeys.length || val.onKeys.includes(key)).map(v => v.handler(key, value))
        },
        onChange: (handler: Function, onKeys: string[] = []) => handlersWithState[eventName].stateListeners.push({handler, onKeys}),
        getVal: <T>(key: string) => handlersWithState[eventName].state[key] as T,
        src: () => src
    })
    if (!handlersWithState[eventName].registeredListenerRef) {
        const handler = (payload: any) => {
            const state = wrappedState(source)
            handlersWithState[eventName].handlers.forEach(hndlr => {
                hndlr(state, payload)
            })
        }
        handlersWithState[eventName].registeredListenerRef = handler
        onNet(eventName, handler)
    }
    const idx = handlersWithState[eventName].handlers.push(handler) - 1
    return () => unregisterEventHandler(eventName, idx, false)
}

/**
 * Event handler without state
 */
export function useDumbEvent<T extends unknown>(eventName: string, handler: (src: number, payload: T) => void) {
init(eventName, true)
    if (!handlersWithoutState[eventName].registeredListenerRef) {
        const handler = (payload: any) => {
            const src = source
            handlersWithoutState[eventName].handlers.forEach(hndlr => {
                hndlr(src, payload)
            })
        }
        handlersWithoutState[eventName].registeredListenerRef = handler
        onNet(eventName, handler)
    }
    const idx = handlersWithoutState[eventName].handlers.push(handler) - 1
    return () => unregisterEventHandler(eventName, idx, true)
}