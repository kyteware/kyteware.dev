import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';

interface GumballWrapperProps {
    dropPressed: boolean,
    setDropPressed: (a: boolean) => void
}

export default function GumballWrapper({ dropPressed, setDropPressed }: GumballWrapperProps) {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

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
        const ret = dropPressed;
        if (dropPressed == true) {
            setDropPressed(false);
        }

        return ret;
    }, [dropPressed]);

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