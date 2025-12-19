import { useEffect, useState, type ReactNode } from "react";
import type { Gumballs } from "./data";
import './Controls.css';
import Markdown from "react-markdown";

interface ControlsProps {
    gumballs: Gumballs | null,
    triggerDrop: React.Dispatch<React.SetStateAction<number>>,
    triggerEject: React.Dispatch<React.SetStateAction<number>>,
    lastDropped: number | null,
    loadingState: string | null
}

enum Stage {
    LOADING = "LOADING",
    READY = "READY",
    DROPPING = "DROPPING",
    FACT_DISPLAYED = "FACT_DISPLAYED",
}

export default function Controls({ gumballs, triggerDrop, triggerEject, lastDropped, loadingState }: ControlsProps) {
    const [stage, setStage] = useState(Stage.LOADING);

    // must be in here for change detection
    useEffect(() => {
        if (lastDropped !== null) {
            setStage(Stage.FACT_DISPLAYED);
        }
    }, [lastDropped]);

    switch (stage) {
        case Stage.LOADING:
            return <LoadingStage setStage={setStage} loadingState={loadingState}/>
        case Stage.READY:
            return <ReadyStage setStage={setStage} triggerDrop={triggerDrop}/>
        case Stage.DROPPING:
            return <DroppingStage/>
        case Stage.FACT_DISPLAYED:
            return <FactDisplayedStage setStage={setStage} gumballs={gumballs} lastDropped={lastDropped} triggerEject={triggerEject}/>
        default:
            console.error("UNKNOWN STAGE AHHH")
            break;
    }
}

function formatControls(content: ReactNode, buttonText: string, buttonDisabled: boolean, onClick: () => void) {
    return <div id="controls">
        <div id="control-panel">
            <div className="controlsInner">
                {content}
            </div>
            <button className={"controlButton " + (buttonDisabled ? "buttonDisabled" : "")} onClick={onClick}>
                {buttonText}        
            </button>
        </div>
    </div>;
}

function formatMessage(text: string) {
    return <div className="controlMessageContainer">
        <p>{text}</p>
    </div>
}

interface LoadingStageData {
    setStage: (s: Stage) => void,
    loadingState: string | null
}

function LoadingStage({ setStage, loadingState }: LoadingStageData) {
    useEffect(() => {
        if (loadingState === null) {
            setStage(Stage.READY);
        }
    }, [loadingState]);

    return formatControls(
        formatMessage("Loading..."),
        "Drop",
        true,
        () => {}
    )
}

interface ReadyStageData {
    setStage: (s: Stage) => void,
    triggerDrop: React.Dispatch<React.SetStateAction<number>>
}

function ReadyStage({ setStage, triggerDrop }: ReadyStageData) {
    return formatControls(
        formatMessage("Ready to drop!"),
        "Drop",
        false,
        () => {
            triggerDrop((old) => old + 1);
            setStage(Stage.DROPPING)
        }
    )
}


function DroppingStage() {
    return formatControls(
        formatMessage("Dropping..."),
        "Dropping...",
        true,
        () => {}
    )
}

interface FactDisplayedStageData {
    setStage: (s: Stage) => void,
    gumballs: Gumballs | null,
    lastDropped: number | null
    triggerEject: React.Dispatch<React.SetStateAction<number>>,
}

function FactDisplayedStage({ setStage, gumballs, lastDropped, triggerEject }: FactDisplayedStageData) {
    const toDisplay = gumballs!.find(lastDropped!);
    return formatControls(
        <Markdown>{toDisplay.content}</Markdown>,
        "Discard",
        false,
        () => {
            triggerEject((old) => old + 1);
            setStage(Stage.READY);
        }
    )
}
