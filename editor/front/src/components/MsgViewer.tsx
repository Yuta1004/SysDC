import { useContext } from "react";

import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import { ShowOkContext, ShowErrContext } from "../App";

const MsgViewer = () => {
    const [okMsg, showOkMsg] = useContext(ShowOkContext);
    const [errMsg, showErrMsg] = useContext(ShowErrContext);

    return (<>
        <Snackbar
            open={ okMsg !== "" }
            autoHideDuration={6000}
            onClose={ () => showOkMsg("") }
            anchorOrigin={{ vertical: "top", horizontal: "center"}}
            sx={{
                zIndex: 9999
            }}
        >
            <Alert
                onClose={ () => showOkMsg("") }
                severity="success"
            >
                { okMsg }
            </Alert>
        </Snackbar>
        <Snackbar
            open={ errMsg !== "" }
            autoHideDuration={6000}
            onClose={ () => showErrMsg("") }
            anchorOrigin={{ vertical: "top", horizontal: "center"}}
            sx={{
                zIndex: 9999
            }}
        >
            <Alert
                onClose={ () => showErrMsg("") }
                severity="error"
            >
                { errMsg }
            </Alert>
        </Snackbar>
    </>);
};

export default MsgViewer;
