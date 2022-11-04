import AceEditor from "react-ace";
import "brace/theme/eclipse";

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
        <AceEditor
            theme="eclipse"
            showGutter={true}
            showPrintMargin={false}
            highlightActiveLine={true} 
            style={ props.style }
            onLoad={ setSyntaxHighlight }
        />
    );
};

export default Editor;
