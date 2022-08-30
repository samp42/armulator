import Editor from "@monaco-editor/react";
import './CodeEditor.css';
import {useEffect, useState} from "react";
import init, {get_person, parse, run} from "../../../pkg";

export function CodeEditor() {
    const [code, setCode] = useState("// type your code...");
    const handleEditorChange = (newCode: any) => {
        setCode(newCode);
    };

    useEffect(() => {
        init().then(_ => { /* do nothing */ });
    }, []);

    function test() {
        const data = code.split('\n').filter(r => !r.startsWith('//'))[0];
        console.log(parse(data))
    }

    function logPerson() {
        let person = get_person();
        setCode(code + `\n${person.name} ${person.age} ${person.xyz}`);
    }

    function runCode() {
        console.log(run());
    }

    return (
        <div>
            <h1 className={"title"}>Armulator</h1>
            <div style={{height: "45px", display: "flex", justifyContent: "center"}}>
                <button className={"btnRun"} onClick={runCode}>Run</button>
            </div>

            {/*<button onClick={logPerson}>test struct</button>
            <button onClick={test}>Test</button>*/}
            <div>
                <Editor
                    height="70vh"
                    language={"cpp"}
                    theme={"vs-dark"}
                    value={code}
                    defaultValue="// some comment"
                    onChange={handleEditorChange}
                />
            </div>
        </div>
    );
}

export default CodeEditor;
