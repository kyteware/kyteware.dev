import { useState } from 'react';
import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'

export default function App() {
    const [dropTrigger, triggerDrop] = useState(0);

    return (
        <div id="siteLayout">
            <div id="headerbar">
                <p>headerbar</p>
            </div>
            <Controls triggerDrop={triggerDrop}/>
            <GumballWrapper dropTrigger={dropTrigger}/>
            <div id="statusbar">
                <p>statusbar</p>
            </div>
        </div>
    )
}
