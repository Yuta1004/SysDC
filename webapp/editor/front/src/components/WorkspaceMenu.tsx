import { useContext } from "react";

import Backdrop from "@mui/material/Backdrop";
import CloseIcon from "@mui/icons-material/Close";
import IconButon from "@mui/material/IconButton";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import TextField from "@mui/material/TextField";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import Card from "@mui/material/Card";

import { WorkspaceContext } from "../App";

interface WorkspaceMenuProps {
    onWorkspaceOpen: () => void,
    onWorkSpaceCreate: () => void
}

const WorkspaceMenu = (props: WorkspaceMenuProps) => {
    const [[stat, workspace], showWorkspaceMenu] = useContext(WorkspaceContext);

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
                    onClick={ () => showWorkspaceMenu([false, workspace])}
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
                        label="Workspace ID"
                        variant="standard"
                        InputLabelProps={{ shrink: true }}
                        sx={{ width: "100%"}}
                    />
                    <Button onClick={ props.onWorkspaceOpen }>
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
                            ※ああああああああああああああああああ<br/>
                            ※ああああああああああああああああああ<br/>
                            ※ああああああああああああああああああ<br/>
                    </Card>
                    <Button onClick={ props.onWorkSpaceCreate }>
                        規約に同意して作成
                    </Button>
                </div>
            </Paper>
        </Backdrop>
    );
};

export default WorkspaceMenu;
