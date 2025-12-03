import { useEffect, useState } from 'react';
import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'
import { Gumballs } from './model';

export default function App() {
    const [dropTrigger, triggerDrop] = useState(0);
    const [gumballs, setGumballs] = useState<Gumballs | null>(null);

    // retrieve gumballs
    useEffect(() => {
        const fetchGumballs = async () => {
            try {
                const response = await fetch('./gumballs.json');

                if (!response.ok) {
                    throw new Error('COULDN"T GET GUMBALLS GAHHHH status: ${response.status}');
                }

                const result = new Gumballs(await response.json());

                setGumballs(result);
            } catch (err) {
                console.error(err);
            }
        }

        fetchGumballs();
    }, [])

    return (
        <div id="siteLayout">
            <div id="headerbar">
                <p>headerbar</p>
            </div>
            <Controls triggerDrop={triggerDrop}/>
            <GumballWrapper dropTrigger={dropTrigger} gumballs={gumballs}/>
            <div id="statusbar">
                <p>statusbar</p>
            </div>
        </div>
    )
}
