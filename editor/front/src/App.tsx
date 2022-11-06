import React, { useEffect, useState, createContext } from "react";

import Box from "@mui/material/Box";

import Header from "./components/Header";
import FileExplorer from "./components/FileExplorer";
import Editor from "./components/Editor";
import ToolViewer from "./components/ToolViewer";
import MyFileSystem from "./filesystem/MyFileSystem";
import init, { Parser } from "sysdc_core";
import MsgViewer from "./components/MsgViewer";

type SContextType<T> = [T, React.Dispatch<React.SetStateAction<T>>];

// ファイルシステム用Context
export const FSContext = createContext({} as MyFileSystem);
export const TargetFileContext = createContext({} as SContextType<string>);

// メッセージ表示用Context
export const ShowOkContext = createContext({} as SContextType<string>);
export const ShowErrContext = createContext({} as SContextType<string>);

const App = () => {
    const [fs, _setFs] = useState(new MyFileSystem());
    const [targetFile, setTargetFile] = useState("/design.def");

    const [okMsg, showOkMsg] = useState("");
    const [errMsg, showErrMsg] = useState("");

    const parse = () => {
        const parser = Parser.new();
        try {
            fs.readAll().map(f => parser.parse(f.name, f.body) );
            parser.check();
        } catch (err) {
            showErrMsg(err+"");
            return;
        }
        showOkMsg("OK");
    };

    useEffect(() => {
        init();
        fs.mkfile("/design.def", "unit design;");
    }, []);

    return (
        <Box
            sx={{
                display: "flex",
                flexDirection: "column",
                width: "100vw",
                height: "100vh"
            }} 
        >
            <Header
                onParseClick={ parse }
            />
            <Box
                sx={{
                    display: "flex",
                    flexDirection: "row",
                    width: "100%",
                    height: "100%"
                }}
            >
                <FSContext.Provider value={ fs }>
                    <TargetFileContext.Provider value={[ targetFile, setTargetFile ]}>
                        <FileExplorer
                            width="220px"
                        />
                        <Editor/>
                    </TargetFileContext.Provider>
                </FSContext.Provider>
            </Box>
            <ToolViewer/>
            <ShowOkContext.Provider value={[ okMsg, showOkMsg ]}>
                <ShowErrContext.Provider value={[ errMsg, showErrMsg ]}>
                    <MsgViewer/>
                </ShowErrContext.Provider>
            </ShowOkContext.Provider>
        </Box>
    );
};

export default App;
