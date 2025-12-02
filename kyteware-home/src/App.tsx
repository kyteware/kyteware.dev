import { useState } from 'react';
import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'

export default function App() {
    const [dropPressed, setDropPressed] = useState(false);

    return (
        <div id="siteLayout">
            <div id="headerbar">
                <p>headerbar</p>
            </div>
            <Controls setDropPressed={setDropPressed}/>
            <GumballWrapper dropPressed={dropPressed} setDropPressed={setDropPressed}/>
            <div id="statusbar">
                <p>statusbar</p>
            </div>
        </div>
    )
}
