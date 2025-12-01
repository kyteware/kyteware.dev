import { useCallback, useEffect, useState } from 'react'
import './App.css'
import * as wasm from 'gumballs';

function App() {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

    const getGumballsCallback = useCallback(() => {
        console.log("hi");

        return {
            "personal_projects": [
                0, 1, 2, 3, 4
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
    });

    useEffect(() => {
        if (isWasmLoaded) {
            wasm.run();
        }
    }, [isWasmLoaded])

    return (
        <>
            <canvas id="gumball-canvas" />
        </>
    )
}

export default App
