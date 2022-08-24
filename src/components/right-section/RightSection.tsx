import './RightSection.css';
import Diodes from "./diodes/Diodes";
import CommandPalette from "./command-palette/CommandPalette";

function RightSection() {
    return (
        <div className="right-section">
            <CommandPalette></CommandPalette>
            <Diodes diodes={[true, false, true, false, true, false, true, false]} setDiode={() => {}}></Diodes>
        </div>
    );
}

export default RightSection;
