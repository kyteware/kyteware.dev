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
    const [loadingState, setLoadingState] = useState<string | null>("WASM loading");

    const doneDroppingCallback = useCallback((id: number) => {
        setLastDropped(id);
    }, [setLastDropped]);

    const loadingProgress = (progress: string) => {
        setLoadingState(progress);
    }

    const doneLoading = () => {
        setLoadingState(null);
    }

    // register dropped callback
    useEffect(() => {
        (window as any).doneDropping = doneDroppingCallback;
        (window as any).loadingProgress = loadingProgress;
        (window as any).doneLoading = doneLoading;

        return () => {
            (window as any).doneDropping = undefined;
            (window as any).loadingProgress = undefined;
            (window as any).doneLoading = undefined;
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

    const is_loading = loadingState !== null;

    // prayers \o/
    return (
        <div id="gumball-container">
            <div className="square-constraint">
                <div id="gumball-wrapper">
                    <canvas id="gumball-canvas" style={is_loading ? { display: "none" } : {}} tabIndex={-1}/>
                </div>
            </div>

            <LoadingDisplayMaybe loadingState={loadingState}/>
        </div>
    )
}

function LoadingDisplayMaybe({ loadingState }: { loadingState: string | null }) {
    return loadingState === null 
        ? <></> 
        : (
            <div className="loading_cover">
                <p>{loadingState}</p>
            </div>
        )
}