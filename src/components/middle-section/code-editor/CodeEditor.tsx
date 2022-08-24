import Editor from "@monaco-editor/react";
import {useEffect, useState} from "react";
import init, {add, parse} from "wasm-lib";

export function CodeEditor() {
    const [code, setCode] = useState("// type your code...");
    const handleEditorChange = (code: any) => {
        setCode(code);
    };

    useEffect(() => {
        init().then(_ => { /* do nothing */ });
    }, []);

    function test() {
        const data = code.split('\n').filter(r => !r.startsWith('//'))[0];
        console.log(parse(data))
    }

    return (
        <div>
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
