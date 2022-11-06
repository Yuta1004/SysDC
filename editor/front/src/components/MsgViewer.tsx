import { useContext } from "react";

import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import { ShowOkContext, ShowErrContext } from "../App";

const MsgViewer = () => {
    const [showOk, setShowOk] = useContext(ShowOkContext);
    const [showErr, setShowErr] = useContext(ShowErrContext);

    return (<>
        <Snackbar
            open={ showOk[0] }
            autoHideDuration={6000}
            onClose={ () => setShowOk([false, ""]) }
            anchorOrigin={{ vertical: "top", horizontal: "center"}}
            style={{
                zIndex: 9999
            }}
        >
            <Alert
                onClose={ () => setShowOk([false, ""]) }
                severity="success"
            >
                { showOk[1] }
            </Alert>
        </Snackbar>
        <Snackbar
            open={ showErr[0] }
            autoHideDuration={6000}
            onClose={ () => setShowErr([false, ""]) }
            anchorOrigin={{ vertical: "top", horizontal: "center"}}
            style={{
                zIndex: 9999
            }}
        >
            <Alert
                onClose={ () => setShowErr([false, ""]) }
                severity="error"
            >
                { showErr[1] }
            </Alert>
        </Snackbar>
    </>);
};

export default MsgViewer;

