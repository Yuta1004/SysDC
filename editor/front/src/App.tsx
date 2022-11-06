import { useEffect, useState } from "react";

import Box from "@mui/material/Box";
import Snackbar from "@mui/material/Snackbar";
import Alert from "@mui/material/Alert";

import Header from "./components/Header";
import FileExplorer from "./components/FileExplorer";
import Editor from "./components/Editor";
import ToolViewer from "./components/ToolViewer";
import MyFileSystem from "./filesystem/MyFileSystem";
import init, { Parser } from "sysdc_core";

const App = () => {
    const [fs, _setFs] = useState(new MyFileSystem());
    const [targetFile, setTargetFile] = useState("/design.def");

    const [showOk, setShowOk] = useState(false);
    const [showErr, setShowErr] = useState(false);
    const [errMsg, setErrMsg] = useState("");

    const parse = () => {
        const parser = Parser.new();
        try {
            fs.readAll().map(f => parser.parse(f.name, f.body) );
            parser.check();
        } catch (err) {
            setErrMsg(err+"");
            setShowErr(true);
            return;
        }
        setShowOk(true);
    };

    useEffect(() => {
        init();
        fs.mkfile("/design.def", "unit design;");
    }, []);

    return (
        <Box
            style={{
                display: "flex",
                flexDirection: "column",
                width: "100vw",
                height: "100vh"
            }} 
        >
            <Header
                onParseClick={ parse }
                style={{
                    flex: 1
                }}
            />
            <Box
                style={{
                    display: "flex",
                    flexDirection: "row",
                    width: "100%",
                    height: "100%"
                }}
            >
                <FileExplorer
                    fs={ fs }
                    onSelect={ path => setTargetFile(path) }
                    style={{
                        padding: 0,
                        flex: 1,
                        minWidth: "220px"
                    }}
                />
                <Editor
                    fs={ fs }
                    targetFile={ targetFile }
                    style={{
                        width: "100%",
                        height: "100%"
                    }} 
                />
            </Box>
            <ToolViewer/>
            <Snackbar
                open={ showOk }
                autoHideDuration={6000}
                onClose={ () => setShowOk(false) }
                anchorOrigin={{ vertical: "top", horizontal: "center"}}
                style={{
                    zIndex: 9999
                }}
            >
                <Alert
                    onClose={ () => setShowOk(false) }
                    severity="success"
                >
                    OK
                </Alert>
            </Snackbar>
            <Snackbar
                open={ showErr }
                autoHideDuration={6000}
                onClose={ () => setShowErr(false) }
                anchorOrigin={{ vertical: "top", horizontal: "center"}}
                style={{
                    zIndex: 9999
                }}
            >
                <Alert
                    onClose={ () => setShowErr(false) }
                    severity="error"
                >
                    { errMsg }
                </Alert>
            </Snackbar>
        </Box>
    );
};

export default App;
