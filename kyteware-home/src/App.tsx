import { useEffect, useState } from 'react';
import './App.css'
import Controls from './Controls'
import GumballWrapper from './GumballWrapper'
import { Gumballs } from './data';
import Headerbar from './Headerbar';

export default function App() {
    const [gumballs, setGumballs] = useState<Gumballs | null>(null);

    const [dropTrigger, triggerDrop] = useState(0);
    const [ejectTrigger, triggerEject] = useState(0);
    const [lastDropped, setLastDropped] = useState<number | null>(null);

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
            <Headerbar/>
            <Controls gumballs={gumballs} triggerDrop={triggerDrop} triggerEject={triggerEject} lastDropped={lastDropped}/>
            <GumballWrapper gumballs={gumballs} dropTrigger={dropTrigger} ejectTrigger={ejectTrigger} setLastDropped={setLastDropped}/>
        </div>
    )
}
