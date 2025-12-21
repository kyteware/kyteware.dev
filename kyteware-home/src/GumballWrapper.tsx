import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';
import type { Gumballs } from "./data";
import './GumballWrapper.css';
import { Stage } from "./App";

interface GumballWrapperProps {
    gumballs: Gumballs | null
    dropTrigger: number,
    ejectTrigger: number,
    setLastDropped: (id: number) => void,
    setStage: (stage: Stage) => void
}

export default function GumballWrapper({ gumballs, dropTrigger, ejectTrigger, setLastDropped, setStage }: GumballWrapperProps) {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);
    const [loadingState, setLoadingState] = useState<string | null>("WASM loading");

    const doneDroppingCallback = useCallback((id: number) => {
        setLastDropped(id);
        setStage(Stage.FACT_DISPLAYED)
    }, [setLastDropped]);

    const loadingProgress = (progress: string) => {
        setLoadingState(progress);
    }

    const doneLoading = () => {
        setLoadingState(null);
        setStage(Stage.FILLING);
    }

    const doneFilling = () => {
        setStage(Stage.READY);
    }

    // register dropped callback
    useEffect(() => {
        (window as any).doneDropping = doneDroppingCallback;
        (window as any).loadingProgress = loadingProgress;
        (window as any).doneLoading = doneLoading;
        (window as any).doneFilling = doneFilling;

        return () => {
            (window as any).doneDropping = undefined;
            (window as any).loadingProgress = undefined;
            (window as any).doneLoading = undefined;
            (window as any).doneFilling = undefined;
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
    }, [isWasmLoaded]);

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
            <div className="loading-cover">
                <div className="progress-container">
                    {loadingState.split("\n").map((line, index) => <p key={index}>{line}</p>)}
                </div>
                <p className="technical-credits">
                    Implemented with Bevy in Rust, ported with wasm-bindgen ♥️
                </p>
            </div>
        )
}