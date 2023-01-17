import { useEffect, useState, useContext } from "react";

import AceEditor from "react-ace";
import "brace/theme/eclipse";
import Box from "@mui/material/Stack";
import Stack from "@mui/material/Stack";
import Chip from "@mui/material/Chip";
import IconButton from "@mui/material/IconButton";
import SaveIcon from "@mui/icons-material/Save";

import SysDCSyntaxHighlight from "../ace_custom/SysDCSyntaxHighlight";
import { FSContext, TargetFileContext } from "../App";

const Editor = () => {
    const fs = useContext(FSContext);
    const [targetFile, _setTargetFile] = useContext(TargetFileContext);

    const [code, setCode] = useState("");
    const [statStr, setStatStr] = useState("");

    const startEditing = (newCode: string) => {
        setCode(newCode);
        setStatStr(targetFile + " を編集中…");
    };

    const saveEditing = () => {
        fs.mkfile(targetFile, code);
        setStatStr(targetFile + " を保存しました");
    };

    const setSyntaxHighlight = (editor: any) => {
        let session = editor.getSession();
        session.$mode.$highlightRules.$rules = SysDCSyntaxHighlight;
        session.$mode.$tokenizer = null;
        session.bgTokenizer.setTokenizer(editor.session.$mode.getTokenizer());
        session.bgTokenizer.start(0);
    };

    useEffect(() => {
        const result = fs.read(targetFile);
        if (result !== undefined) {
            setCode(result);
            setStatStr(targetFile + " をオープン");
        }
    }, [fs, targetFile]);

    return (
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
    );
};

export default Editor;
