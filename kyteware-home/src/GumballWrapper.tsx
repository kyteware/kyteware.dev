import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';

export default function GumballWrapper() {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

    const getGumballsCallback = useCallback(() => {
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

    useEffect(() => {
        (window as any).getGumballs = getGumballsCallback;

        wasm.default()
            .then(() => {
                console.log("gumballs module initialized");
                setIsWasmLoaded(true);
            })
            .catch(error => {
                console.error("couldn't load gumballs wasm module: ", error);
            });

        return () => {
            (window as any).getGumballs = undefined;
        }
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