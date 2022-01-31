const handlersWithState: Record<string, { registeredListenerRef?: Function, handlers: Function[], state: Record<string, any>, stateListeners: {onKeys: string[], handler: Function}[]}> = {}
const init = (name: string) => {
    if (!handlersWithState[name]) {
        handlersWithState[name] = {handlers: [], state: {}, stateListeners: []}
    }
}

interface State {
    attach: (key: string, value: any) => void,
    onChange: <T>(handler: (key: string, value: T) => void, onKeys?: string[]) => void
    getKey: <T>(key: string) => T
    src: () => number

}

function unregisterEventHandler(name: string, idx: number) {
    delete handlersWithState[name].handlers[idx]
    handlersWithState[name].handlers = handlersWithState[name].handlers.filter(val => val)
}


export function useNetEvent<T extends unknown, R extends void = void>(eventName: string, handler: (state: State, payload: T) => Promise<R> | R) {
    init(eventName)
    const wrappedState = (src: number): State => ({
        attach: (key: string, value: any) => {
            handlersWithState[eventName].state[key] = value
            handlersWithState[eventName].stateListeners.filter(val => !val.onKeys || val.onKeys.includes(key)).map(v => v.handler(key, value))
        },
        onChange: (handler: Function, onKeys: string[] = []) => handlersWithState[eventName].stateListeners.push({handler, onKeys}),
        getKey: <T>(key: string) => handlersWithState[eventName].state[key] as T,
        src: () => src
    })
    if (!handlersWithState[eventName].registeredListenerRef) {
        const handler = (payload: any) => {
            const state = wrappedState(source)
            handlersWithState[eventName].handlers.map(hndlr => {
                hndlr(state, payload)
            })
        }
        handlersWithState[eventName].registeredListenerRef = handler
        onNet(eventName, handler)
    }
    return () => unregisterEventHandler(eventName, handlersWithState[eventName].handlers.push(handler))
}