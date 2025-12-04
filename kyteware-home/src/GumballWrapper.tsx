import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';
import type { Gumballs } from "./model";

interface GumballWrapperProps {
    gumballs: Gumballs | null
    dropTrigger: number,
    setLastDropped: (id: number) => void,
}

export default function GumballWrapper({ gumballs, dropTrigger, setLastDropped }: GumballWrapperProps) {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

    const droppedCallback = useCallback((id: number) => {
        setLastDropped(id);
    }, [setLastDropped]);

    // register dropped callback
    useEffect(() => {
        (window as any).dropped = droppedCallback;

        return () => {
        (window as any).dropped = undefined;
        }
    }, [droppedCallback]);

    // init wasm
    useEffect(() => {
        wasm.default()
            .then(() => {
                console.log("gumballs module initialized");
                wasm.run();
                setIsWasmLoaded(true);
             })
            .catch(error => {
                console.error("couldn't load gumballs wasm module: ", error);
            });
    }, []);

    // send gumballs when ready
    useEffect(() => {
        if (isWasmLoaded && gumballs !== null) {
            wasm.gumballs_available(gumballs.gumballs);
        }
    }, [isWasmLoaded, gumballs]);

    // drop on trigger
    useEffect(() => {
        if (!isWasmLoaded) {
            return;
        }

        wasm.drop_gumball();
    }, [dropTrigger])

    return (
        <div id="gumball-wrapper">
            <canvas id="gumball-canvas"/>
        </div>
    )
}