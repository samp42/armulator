import './MiddleSection.css';
import CodeEditor from "./code-editor/CodeEditor";
import CompilationConsole from "./compilation-console/CompilationConsole";

function MiddleSection() {
    return (
        <div className="middle-section">
            <CodeEditor></CodeEditor>
            <CompilationConsole></CompilationConsole>
        </div>
    );
}

export default MiddleSection;
