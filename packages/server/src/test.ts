import {useServerState, useStateEffect} from "./state";
import useArray from "./arrays";


export function test() {
    useStateEffect(() => {
        console.log("HELLO")
        const state = useServerState()
    })
    console.log(useArray([10, 10]))

}