import { useState } from 'react';
import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'
import { Gumballs } from './data';
import Headerbar from './Headerbar';

export default function App() {
    const [loadingState, setLoadingState] = useState<string | null>("WASM loading");
    const [dropTrigger, triggerDrop] = useState(0);
    const [ejectTrigger, triggerEject] = useState(0);
    const [lastDropped, setLastDropped] = useState<number | null>(null);

    const gumballs = new Gumballs();

    return (
        <div id="siteLayout">
            <Headerbar/>
            <Controls gumballs={gumballs} triggerDrop={triggerDrop} triggerEject={triggerEject} lastDropped={lastDropped} loadingState={loadingState}/>
            <GumballWrapper gumballs={gumballs} dropTrigger={dropTrigger} ejectTrigger={ejectTrigger} setLastDropped={setLastDropped} loadingState={loadingState} setLoadingState={setLoadingState}/>
        </div>
    )
}
