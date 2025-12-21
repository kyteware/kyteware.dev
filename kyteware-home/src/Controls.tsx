import { useEffect, useState, type ReactNode } from "react";
import type { Gumballs } from "./data";
import './Controls.css';
import Markdown from "react-markdown";

interface ControlsProps {
    gumballs: Gumballs | null,
    triggerDrop: React.Dispatch<React.SetStateAction<number>>,
    triggerEject: React.Dispatch<React.SetStateAction<number>>,
    lastDropped: number | null,
    loadingState: string | null,
    doneFilling: boolean
}

enum Stage {
    LOADING = "LOADING",
    FILLING = "FILLING",
    READY = "READY",
    DROPPING = "DROPPING",
    FACT_DISPLAYED = "FACT_DISPLAYED",
    DONE = "DONE"
}

export default function Controls({ gumballs, triggerDrop, triggerEject, lastDropped, loadingState, doneFilling }: ControlsProps) {
    const [stage, setStage] = useState(Stage.LOADING);
    const [numDropped, setNumDropped] = useState(0);

    // must be in here for change detection
    useEffect(() => {
        if (lastDropped !== null) {
            setStage(Stage.FACT_DISPLAYED);
        }
    }, [lastDropped]);

    switch (stage) {
        case Stage.LOADING:
            return <LoadingStage setStage={setStage} loadingState={loadingState}/>
        case Stage.FILLING:
            return <FillingStage setStage={setStage} doneFilling={doneFilling}/>
        case Stage.READY:
            return <ReadyStage setStage={setStage} triggerDrop={triggerDrop} setNumDropped={setNumDropped}/>
        case Stage.DROPPING:
            return <DroppingStage/>
        case Stage.FACT_DISPLAYED:
            return <FactDisplayedStage setStage={setStage} gumballs={gumballs} lastDropped={lastDropped} triggerEject={triggerEject} numDropped={numDropped}/>
        case Stage.DONE:
            return <DoneStage/>
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
            setStage(Stage.FILLING);
        }
    }, [loadingState]);

    return formatControls(
        formatMessage("Loading..."),
        "Drop",
        true,
        () => {}
    )
}

interface FillingStageData {
    setStage: (s: Stage) => void,
    doneFilling: boolean
}

function FillingStage({ setStage, doneFilling }: FillingStageData) {
    useEffect(() => {
        if (doneFilling === true) {
            setStage(Stage.READY);
        }
    }, [doneFilling]);

    return formatControls(
        formatMessage("Filling machine..."),
        "Drop",
        true,
        () => {}
    )
}

interface ReadyStageData {
    setStage: (s: Stage) => void,
    triggerDrop: React.Dispatch<React.SetStateAction<number>>
    setNumDropped: React.Dispatch<React.SetStateAction<number>>
}

function ReadyStage({ setStage, triggerDrop, setNumDropped }: ReadyStageData) {
    return formatControls(
        formatMessage("Ready to drop!"),
        "Drop",
        false,
        () => {
            setNumDropped((old) => old + 1); // wait a minute...
            triggerDrop((old) => old + 1); // i think this might be redundant guys
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
    numDropped: number
}

function FactDisplayedStage({ setStage, gumballs, lastDropped, triggerEject, numDropped }: FactDisplayedStageData) {
    const toDisplay = gumballs!.find(lastDropped!);
    return formatControls(
        <Markdown>{toDisplay.content}</Markdown>,
        "Discard",
        false,
        () => {
            triggerEject((old) => old + 1);
            if (gumballs!.gumballs.length > numDropped) {
                setStage(Stage.READY);
            } else {
                setStage(Stage.DONE);
            }
        }
    )
}

function DoneStage() {
    return formatControls(
        formatMessage("No more gumballs!"),
        "Drop",
        true,
        () => {}
    )
}
