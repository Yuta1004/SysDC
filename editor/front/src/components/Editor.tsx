import { useEffect, useState } from "react";

import AceEditor from "react-ace";
import "brace/theme/eclipse";
import Box from "@mui/material/Stack";
import Stack from "@mui/material/Stack";
import Chip from "@mui/material/Chip";
import IconButton from "@mui/material/IconButton";
import SaveIcon from "@mui/icons-material/Save";

import SysDCSyntaxHighlight from "../ace_custom/SysDCSyntaxHighlight";
import MyFileSystem from "../filesystem/MyFileSystem";

interface EditorProps {
    style: React.CSSProperties | undefined,
    fs: MyFileSystem,
    targetFile: string
}

const Editor = (props: EditorProps) => {
    const [code, setCode] = useState("");
    const [statStr, setStatStr] = useState("");

    useEffect(() => {
        const result = props.fs.read(props.targetFile);
        if (result !== undefined) {
            setCode(result);
            setStatStr(props.targetFile + " をオープン");
        }
    }, [props.fs, props.targetFile]);

    const startEditing = (newCode: string) => {
        setCode(newCode);
        setStatStr(props.targetFile + " を編集中…");
    };

    const saveEditing = () => {
        props.fs.mkfile(props.targetFile, code);
        setStatStr(props.targetFile + " を保存しました");
    };

    const setSyntaxHighlight = (editor: any) => {
        let session = editor.getSession();
        session.$mode.$highlightRules.$rules = SysDCSyntaxHighlight;
        session.$mode.$tokenizer = null;
        session.bgTokenizer.setTokenizer(editor.session.$mode.getTokenizer());
        session.bgTokenizer.start(0);
    };

    return (
        <Box
            style={ props.style } 
        >
            <Box
                sx={{
                    display: "flex",
                    flexDirection: "column",
                    width: "100%",
                    height: "100%"
                }} 
            >
                <Stack
                    direction="row"
                    spacing={0}
                    alignItems="center"
                >
                    <Chip
                        label={ statStr }
                        variant="outlined"
                        size="small"
                        sx={{
                            width: "fit-content"
                        }}
                    />
                    <IconButton
                        size="small"
                        onClick={ saveEditing }
                    >
                        <SaveIcon />
                    </IconButton>
                </Stack>
                <AceEditor
                    value={ code }
                    theme="eclipse"
                    showGutter={true}
                    showPrintMargin={false}
                    highlightActiveLine={true} 
                    onLoad={ setSyntaxHighlight }
                    onChange={ startEditing }
                    style={{
                        width: "100%",
                        height: "100%"
                    }}
                />
            </Box>
        </Box>
    );
};

export default Editor;
