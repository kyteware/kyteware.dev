import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'

export default function App() {
    

    return (
        <div id="siteLayout">
            <div id="headerbar">
                <p>headerbar</p>
            </div>
            <Controls/>
            <GumballWrapper/>
            <div id="statusbar">
                <p>statusbar</p>
            </div>
        </div>
    )
}
