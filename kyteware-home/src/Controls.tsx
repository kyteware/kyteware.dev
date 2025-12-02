import { useCallback } from "react";

export default function Controls({ setDropPressed }: { setDropPressed: (a: boolean) => void }) {
    const buttonCallback = useCallback(() => {
        setDropPressed(true);
    }, [setDropPressed]);

    return (
        <div id="controls">
            <p>controls</p>
            <button onClick={buttonCallback}>drop</button>
        </div>
    );
}