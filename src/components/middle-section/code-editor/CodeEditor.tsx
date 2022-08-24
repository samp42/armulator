import Editor from "@monaco-editor/react";

export function CodeEditor() {
    // const [value, setValue] = useState(code || "");
    //
    // const handleEditorChange = (value) => {
    //     setValue(value);
    //     onChange("code", value);
    // };

    return (
        <div>
            <Editor
                height="100vh"
                width={`100%`}
                language={"cpp"}
                theme={"vs-dark"}
                // value={value}
                defaultValue="// some comment"
                // onChange={handleEditorChange}
            />
        </div>
    );
}

export default CodeEditor;
