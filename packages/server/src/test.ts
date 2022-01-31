import {useServerState, useStateEffect} from "./state";


export function test() {
    useStateEffect(() => {
        console.log("HELLO")
        const state = useServerState()
    })

}