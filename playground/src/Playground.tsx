import React from 'react';
import Editor, { OnMount as EditorDidMount } from "@monaco-editor/react";

const editorStyle = {
    height: '100%',
    width: '50%',
    display: 'inline-block',
}

export const Playground = () => {
    const [input, setInput] = React.useState("");
    const [output, setOutput] = React.useState("");

    // Handle Input Editor Mounting
    const handleEditorDidMount: EditorDidMount = (editor, monaco) => {
        // Subscribe to input text changing
        editor.onDidChangeModelContent(() => setInput(editor.getValue()))
    }

    // Update the Output when input has changed
    React.useEffect(() => {
        setOutput(input);
    }, [input])

    return (
        <div style={{ height: '70vh' }}>
            <div style={editorStyle}>
                <Editor
                    language="lua"
                    options={{
                        minimap: { enabled: false },
                    }}
                    value={`print("Hello world!")`}
                    onMount={handleEditorDidMount}
                />
            </div>
            <div style={editorStyle}>
                <Editor
                    language="lua"
                    options={{
                        minimap: { enabled: false },
                        readOnly: true,
                    }}
                    value={output}
                />
            </div>
        </div>
    )
}