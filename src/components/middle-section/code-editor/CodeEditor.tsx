import Editor from "@monaco-editor/react";
import {useEffect, useState} from "react";
import init, {parse} from "wasm-lib";
import {testMemField} from "../../../../wasm-lib/pkg";

export function CodeEditor() {
    const [value, setValue] = useState("// type your code...");
    const handleEditorChange = (value: any) => {
        setValue(value);
    };

    useEffect(() => {
        init()
    }, []);

    function test() {
        const data = value.split('\n').filter(r => !r.startsWith('//'))[0];
        console.log(parse(data))
    }

    function memStruct() {
        console.log(testMemField());
    }

    return (
        <div>
            <button onClick={test}>Test</button>
            <button onClick={memStruct}>Test Mem Field</button>
            <div>
                <Editor
                    height="100vh"
                    width={`100%`}
                    language={"cpp"}
                    theme={"vs-dark"}
                    value={value}
                    defaultValue="// some comment"
                    onChange={handleEditorChange}
                />
            </div>
        </div>
    );
}

export default CodeEditor;
