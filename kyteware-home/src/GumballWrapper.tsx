import { useCallback, useEffect, useState } from "react";
import init, * as wasm from 'gumballs';
import wasmUrl from 'gumballs/gumballs_bg.wasm?url';
import { Stage, type Gumballs } from "./common";
import './GumballWrapper.css';

interface GumballWrapperProps {
    gumballs: Gumballs
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
    }, [setStage, setLastDropped]);

    const loadingProgress = useCallback((progress: string) => {
        setLoadingState(progress);
    }, [setLoadingState]);

    const doneLoading = useCallback(() => {
        setLoadingState(null);
        setStage(Stage.FILLING);
    }, [setLoadingState, setStage]);

    const doneFilling = useCallback(() => {
        setStage(Stage.READY);
    }, [setStage]);

    // register dropped callback
    useEffect(() => {
        /* eslint-disable @typescript-eslint/no-explicit-any */
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
        /* eslint-enable @typescript-eslint/no-explicit-any */
    }, [doneDroppingCallback, loadingProgress, doneLoading, doneFilling]);

    // init wasm
    useEffect(() => {
        const initWasm = async () => {
            let wasmSource;

            if (import.meta.env.PROD) {
                const response = await fetch(`${wasmUrl}.gz`)!;
                const ds = new DecompressionStream("gzip");
                const decompressedStream = response.body!.pipeThrough(ds);

                wasmSource = new Response(decompressedStream, { headers: { 'Content-Type': 'application/wasm' } });
            } else {
                wasmSource = wasmUrl;
            }

            await init(wasmSource);
        };

        initWasm()
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
        if (isWasmLoaded) {
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