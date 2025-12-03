export default function Controls({ triggerDrop }: { triggerDrop: React.Dispatch<React.SetStateAction<number>> }) {
    let handleClick = () => {
        triggerDrop((old) => old + 1);
    }

    return (
        <div id="controls">
            <p>controls</p>
            <button onClick={handleClick}>drop</button>
        </div>
    );
}