import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';
import type { Gumballs } from "./model";
import './GumballWrapper.css';

interface GumballWrapperProps {
    gumballs: Gumballs | null
    dropTrigger: number,
    ejectTrigger: number,
    setLastDropped: (id: number) => void,
}

export default function GumballWrapper({ gumballs, dropTrigger, ejectTrigger, setLastDropped }: GumballWrapperProps) {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

    const doneDroppingCallback = useCallback((id: number) => {
        setLastDropped(id);
    }, [setLastDropped]);

    // register dropped callback
    useEffect(() => {
        (window as any).doneDropping = doneDroppingCallback;

        return () => {
        (window as any).doneDropping = undefined;
        }
    }, [doneDroppingCallback]);

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
    }, [dropTrigger]);

    useEffect(() => {
        if (!isWasmLoaded) {
            return;
        }

        wasm.discard_gumball();
    }, [ejectTrigger]);

    return (
        <div id="gumball-wrapper">
            <canvas id="gumball-canvas"/>
        </div>
    )
}