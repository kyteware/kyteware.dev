import { useCallback, useEffect, useState } from "react";
import * as wasm from 'gumballs';

interface GumballWrapperProps {
    dropTrigger: number
}

export default function GumballWrapper({ dropTrigger }: GumballWrapperProps) {
    const [isWasmLoaded, setIsWasmLoaded] = useState(false);

    const droppedCallback = useCallback(() => {
        
    }, []);

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
                wasm.gumballs_available([{category: "Event", id: 1}, {category: "PersonalProject", id: 2}]);
            })
            .catch(error => {
                console.error("couldn't load gumballs wasm module: ", error);
            });
    }, []);

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