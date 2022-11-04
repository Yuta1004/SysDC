import { useEffect, useState } from "react";

import AceEditor from "react-ace";
import "brace/theme/eclipse";
import Chip from "@mui/material/Chip";

import SysDCSyntaxHighlight from "../ace_custom/SysDCSyntaxHighlight";
import MyFileSystem from "../filesystem/MyFileSystem";


interface EditorProps {
    style: React.CSSProperties | undefined,
    fs: MyFileSystem,
    targetFile: string
}

const Editor = (props: EditorProps) => {
    const [code, setCode] = useState("");

    useEffect(() => {
        const result = props.fs.read(props.targetFile);
        if (result !== undefined) {
            setCode(result);
        }
    }, [props.fs, props.targetFile]);

    const setSyntaxHighlight = (editor: any) => {
        let session = editor.getSession();
        session.$mode.$highlightRules.$rules = SysDCSyntaxHighlight;
        session.$mode.$tokenizer = null;
        session.bgTokenizer.setTokenizer(editor.session.$mode.getTokenizer());
        session.bgTokenizer.start(0);
    };

    return (
        <div
            style={ props.style } 
        >
            <div
                style={{
                    display: "flex",
                    flexDirection: "column",
                    width: "100%",
                    height: "100%"
                }} 
            >
                <Chip
                    label={ props.targetFile }
                    variant="outlined"
                    size="small"
                    style={{
                        width: "fit-content"
                    }}
                />
                <AceEditor
                    value={ code }
                    theme="eclipse"
                    showGutter={true}
                    showPrintMargin={false}
                    highlightActiveLine={true} 
                    onLoad={ setSyntaxHighlight }
                    style={{
                        width: "100%",
                        height: "100%"
                    }}
                />
            </div>
        </div>
    );
};

export default Editor;
