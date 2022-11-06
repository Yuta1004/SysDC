import React, { useEffect, useState, createContext } from "react";

import Box from "@mui/material/Box";

import Header from "./components/Header";
import FileExplorer from "./components/FileExplorer";
import Editor from "./components/Editor";
import ToolViewer from "./components/ToolViewer";
import MyFileSystem from "./filesystem/MyFileSystem";
import init, { Parser } from "sysdc_core";
import MsgViewer from "./components/MsgViewer";

type ContextType<T> = [T, React.Dispatch<React.SetStateAction<T>>];

// メッセージ表示用Context
export const ShowOkContext = createContext({} as ContextType<[boolean, string]>);
export const ShowErrContext = createContext({} as ContextType<[boolean, string]>);

const App = () => {
    const [fs, _setFs] = useState(new MyFileSystem());
    const [targetFile, setTargetFile] = useState("/design.def");

    const [showOk, setShowOk] = useState<[boolean, string]>([false, ""]);
    const [showErr, setShowErr] = useState<[boolean, string]>([false, ""]);

    const parse = () => {
        const parser = Parser.new();
        try {
            fs.readAll().map(f => parser.parse(f.name, f.body) );
            parser.check();
        } catch (err) {
            setShowErr([true, err+""]);
            return;
        }
        setShowOk([true, "OK"]);
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
            <ShowOkContext.Provider value={[ showOk, setShowOk ]}>
                <ShowErrContext.Provider value={[ showErr, setShowErr ]}>
                    <MsgViewer/>
                </ShowErrContext.Provider>
            </ShowOkContext.Provider>
        </Box>
    );
};

export default App;
