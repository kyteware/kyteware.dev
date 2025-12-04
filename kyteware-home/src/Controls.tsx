import { useEffect, useState, type JSX } from "react";
import type { Gumballs } from "./model";

interface ControlsProps {
    gumballs: Gumballs | null,
    triggerDrop: React.Dispatch<React.SetStateAction<number>>,
    triggerEject: React.Dispatch<React.SetStateAction<number>>,
    lastDropped: number | null
}

enum Stage {
    READY = "READY",
    DROPPING = "DROPPING",
    FACT_DISPLAYED = "FACT_DISPLAYED",
}

export default function Controls({ gumballs, triggerDrop, triggerEject, lastDropped }: ControlsProps) {
    const [stage, setStage] = useState(Stage.READY);

    let handleDropPressed = () => {
        triggerDrop((old) => old + 1);
        setStage(Stage.DROPPING);
    }

    useEffect(() => {
        if (lastDropped !== null) {
            setStage(Stage.FACT_DISPLAYED);
        }
    }, [lastDropped]);

    let handleNextPressed = () => {
        triggerEject((old) => old + 1)
        setStage(Stage.READY);
    }

    let inner: JSX.Element = <></>;
    if (stage === Stage.READY) {
        inner = <button onClick={handleDropPressed}>drop</button>
    } else if (stage === Stage.DROPPING) {
        inner = <p>waiting to be finished dropping</p>
    } else if (stage === Stage.FACT_DISPLAYED) {
        const fact = gumballs!.gumballs[lastDropped!];
        inner = (<>
            <h1>fact: {fact.name}</h1>
            <p>description: {fact.description}</p>
            <button onClick={handleNextPressed}>discord</button>
        </>);
    }

    return (
        <div id="controls">
            <p>controls</p>
            {inner}
        </div>
    );
}
