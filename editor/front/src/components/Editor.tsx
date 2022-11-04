import AceEditor from "react-ace";
import "brace/theme/eclipse";
import Chip from "@mui/material/Chip";

import SysDCSyntaxHighlight from "../ace_custom/SysDCSyntaxHighlight";

interface EditorProps {
    style: React.CSSProperties | undefined
}

const Editor = (props: EditorProps) => {
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
                    label="A / B / C / Test.def"
                    variant="outlined"
                    size="small"
                    style={{
                        width: "fit-content"
                    }}
                />
                <AceEditor
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
