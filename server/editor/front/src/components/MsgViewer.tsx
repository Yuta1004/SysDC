import { useContext } from "react";

import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import { MsgContext } from "../App";

const MsgViewer = () => {
    const [[type, msg], showMsg] = useContext(MsgContext);

    return (<>
        <Snackbar
            open={ type === "success" }
            autoHideDuration={6000}
            onClose={ () => showMsg(["", msg]) }
            anchorOrigin={{ vertical: "top", horizontal: "center" }}
            sx={{ zIndex: 9999 }}
        >
            <Alert
                onClose={ () => showMsg(["", msg]) }
                severity="success"
            >
                { msg }
            </Alert>
        </Snackbar>
        <Snackbar
            open={ type === "error" }
            autoHideDuration={6000}
            onClose={ () => showMsg(["", msg]) }
            anchorOrigin={{ vertical: "top", horizontal: "center" }}
            sx={{ zIndex: 9999 }}
        >
            <Alert
                onClose={ () => showMsg(["", msg]) }
                severity="error"
            >
                { msg }
            </Alert>
        </Snackbar>
    </>);
};

export default MsgViewer;
