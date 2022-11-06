import { useContext } from "react";

import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import { MsgContext } from "../App";

const MsgViewer = () => {
    const [[type, msg], showMsg] = useContext(MsgContext);

    return (<>
        <Snackbar
            open={ type !== "" }
            autoHideDuration={6000}
            onClose={ () => showMsg(["", ""]) }
            anchorOrigin={{ vertical: "top", horizontal: "center"}}
            sx={{ zIndex: 9999 }}
        >
            <Alert
                onClose={ () => showMsg(["", ""]) }
                severity={ "success" === type ? type : "error" }
            >
                { msg }
            </Alert>
        </Snackbar>
    </>);
};

export default MsgViewer;
