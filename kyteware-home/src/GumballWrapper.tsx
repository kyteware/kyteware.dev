import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';

export default function GumballWrapper() {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);
    const [hasDropped, setHasDropped] = useState(false);

    const getGumballsCallback = useCallback(() => {
        console.log("hi");
        return {
            "personal_projects": [
                
            ],
            "experiences": [
                5, 6
            ],
            "events": [],
            "tidbits": [
                7, 8, 9
            ]
        }
    }, []);

    const shouldDropCallback = useCallback(() => {
        const ret = !hasDropped;

        if (!hasDropped) {
            setHasDropped(true);
        }

        console.log("hello " + ret);
        return ret;
    }, [hasDropped]);

    const droppedCallback = useCallback(() => {
        
    }, []);

    useEffect(() => {
        (window as any).getGumballs = getGumballsCallback;
        (window as any).shouldDrop = shouldDropCallback;
        (window as any).dropped = droppedCallback;

        return () => {
        (window as any).getGumballs = undefined;
        (window as any).shouldDrop = undefined;
        (window as any).dropped = undefined;
        }
    }, [getGumballsCallback, shouldDropCallback]);

    useEffect(() => {
        wasm.default()
            .then(() => {
                console.log("gumballs module initialized");
                setIsWasmLoaded(true);
            })
            .catch(error => {
                console.error("couldn't load gumballs wasm module: ", error);
            });
    }, []);

    useEffect(() => {
        if (isWasmLoaded) {
            wasm.run();
        }
    }, [isWasmLoaded]);

    return (
        <div id="gumball-wrapper">
            <canvas id="gumball-canvas"/>
        </div>
    )
}