import { useState } from 'react';
import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'
import { Gumballs } from './data';
import Headerbar from './Headerbar';

export enum Stage {
    LOADING = "LOADING",
    FILLING = "FILLING",
    READY = "READY",
    DROPPING = "DROPPING",
    FACT_DISPLAYED = "FACT_DISPLAYED",
    DONE = "DONE"
}

export default function App() {
    const [dropTrigger, triggerDrop] = useState(0);
    const [ejectTrigger, triggerEject] = useState(0);
    const [lastDropped, setLastDropped] = useState<number | null>(null);
    const [stage, setStage] = useState(Stage.LOADING);

    const gumballs = new Gumballs();

    return (
        <div id="siteLayout">
            <Headerbar/>
            <Controls gumballs={gumballs} triggerDrop={triggerDrop} triggerEject={triggerEject} lastDropped={lastDropped} stage={stage} setStage={setStage}/>
            <GumballWrapper gumballs={gumballs} dropTrigger={dropTrigger} ejectTrigger={ejectTrigger} setLastDropped={setLastDropped} setStage={setStage}/>
        </div>
    )
}
