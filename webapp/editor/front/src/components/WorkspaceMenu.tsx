import { useContext, useRef } from "react";

import Backdrop from "@mui/material/Backdrop";
import CloseIcon from "@mui/icons-material/Close";
import IconButon from "@mui/material/IconButton";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import TextField, { TextFieldProps } from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import Card from "@mui/material/Card";

import { WorkspaceContext } from "../App";

interface WorkspaceMenuProps {
    onWorkspaceOpen: (workspace: string) => void,
    onWorkSpaceCreate: () => void
}

const WorkspaceMenu = (props: WorkspaceMenuProps) => {
    const [[stat, ws], showWorkspaceMenu] = useContext(WorkspaceContext);

    const wsInput = useRef<TextFieldProps>(null);

    return (
        <Backdrop
            sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}
            open={ stat }
        >
            <Paper
                elevation={5}
                sx={{
                    position: "relative",
                    width: "40%",
                    padding: "20px"
                }}
            >
                <IconButon
                    onClick={ () => showWorkspaceMenu([false, ws])}
                    sx={{
                        position: "absolute",
                        top: "10px",
                        right: "10px"
                    }}
                >
                    <CloseIcon/>
                </IconButon>
                <h2 style={{ margin: "10px 5px 10px 0px" }}>
                    既存ワークスペースを開く
                </h2>
                <Stack
                    direction="column"
                    spacing={2}
                    sx={{
                        display: "flex",
                        flexDirection: "row",
                    }}
                >
                    <TextField
                        inputRef={ wsInput }
                        label="Workspace ID"
                        variant="standard"
                        InputLabelProps={{ shrink: true }}
                        sx={{ width: "100%"}}
                    />
                    <Button
                        onClick={ () => {
                            showWorkspaceMenu([false, wsInput.current?.value+""]);
                            props.onWorkspaceOpen(wsInput.current?.value+"");
                        }}
                    >
                        開く
                    </Button>
                </Stack>
                <Divider sx={{ margin: "25px 0px 25px 0px" }}/>
                <h2 style={{ margin: "10px 5px 15px 0px" }}>
                    新しいワークスペースを作る
                </h2>
                <div>
                    <Card
                        variant="outlined"
                        sx={{ padding: "10px" }}
                    >
                            ※「作成」ボタンを押した時点での状態が保存されます<br/>
                            ※機密事項を入力したファイルを含むワークスペースを作成しないで下さい<br/>
                    </Card>
                    <Button
                        onClick={ () => {
                            showWorkspaceMenu([false, ws]);
                            props.onWorkSpaceCreate();
                        }}
                    >
                        上記の内容に同意して作成
                    </Button>
                </div>
            </Paper>
        </Backdrop>
    );
};

export default WorkspaceMenu;
