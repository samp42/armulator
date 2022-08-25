import Editor from "@monaco-editor/react";
import {useEffect, useState} from "react";
import init, {get_person, parse} from "../../../pkg";

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

    return (
        <div>
            <button onClick={logPerson}>test struct</button>
            <button onClick={test}>Test</button>
            <div>
                <Editor
                    height="100vh"
                    width={`100%`}
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
