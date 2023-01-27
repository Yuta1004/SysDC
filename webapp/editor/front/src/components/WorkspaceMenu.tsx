import { useContext } from "react";

import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import { WorkspaceContext } from "../App";

const WorkspaceMenu = () => {
    const [[stat, workspace], showWorkspaceMenu] = useContext(WorkspaceContext);

    return (<>
        <Snackbar
            open={ stat }
            autoHideDuration={6000}
            onClose={ () => showWorkspaceMenu([false, workspace[1]]) }
            anchorOrigin={{ vertical: "top", horizontal: "center" }}
            sx={{ zIndex: 9999 }}
        >
            <Alert
                onClose={ () => showWorkspaceMenu([false, workspace[1]]) }
                severity="success"
            >
                Select workspace
            </Alert>
        </Snackbar>
    </>);
};

export default WorkspaceMenu;
