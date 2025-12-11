import { useEffect, useState, type JSX } from "react";
import type { Gumballs } from "./model";
import './Controls.css';

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
    let buttonText = "";
    let buttonOnClick = () => {};
    let buttonDisabled = false;

    if (stage === Stage.READY) {
        inner = <p>ready</p>;
        buttonText = "Drop";
        buttonOnClick = handleDropPressed;
    } else if (stage === Stage.DROPPING) {
        inner = <p>waiting to be finished dropping</p>;
        buttonText = "Dropping..."
        buttonOnClick = () => {};
        buttonDisabled = true;
    } else if (stage === Stage.FACT_DISPLAYED) {
        const fact = gumballs!.find(lastDropped!);
        inner = (<>
            <h5>{fact.category}</h5>
            <h3>{fact.name}</h3>
            <p>description: {fact.description}</p>
        </>);
        buttonOnClick = handleNextPressed;
        buttonText = "Discard";
    }

    return (
        <div id="controls">
            <div id="control-panel">
                <div className="controlsInner">
                    {inner}
                </div>
                <button onClick={buttonOnClick} className={"controlButton " + (buttonDisabled ? "buttonDisabled" : "")}>{buttonText}</button>
            </div>
        </div>
    );
}
