import {useServerState, useStateEffect} from "./state";
import useArray from "./arrays";


export function test() {
    useStateEffect(() => {
        console.log("HELLO")
        const state = useServerState()
    })
    const a = new Array(61).fill("test")
    console.log(useArray(a).partition(10, true))
    console.log("-------")
    console.log(useArray(a).partition(10, false))

}