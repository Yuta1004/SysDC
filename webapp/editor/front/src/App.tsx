import axios from "axios";
import React, { useEffect, useState, createContext } from "react";

import Box from "@mui/material/Box";
import Backdrop from "@mui/material/Backdrop";
import CircularProgress from "@mui/material/CircularProgress";

import Header from "./components/Header";
import FileExplorer from "./components/FileExplorer";
import Editor from "./components/Editor";
import ToolViewer from "./components/ToolViewer";
import MyFileSystem from "./filesystem/MyFileSystem";
import init, { Parser } from "sysdc_core";
import MsgViewer from "./components/MsgViewer";
import WorkspaceMenu from "./components/WorkspaceMenu";

type SContextType<T> = [T, React.Dispatch<React.SetStateAction<T>>];

// ファイルシステム用Context
export const FSContext = createContext({} as MyFileSystem);
export const TargetFileContext = createContext({} as SContextType<string>);

// メッセージ表示用Context
export const MsgContext = createContext({} as SContextType<[string, string]>);

// ワークスペースメニュー表示用Context
export const WorkspaceContext = createContext({} as SContextType<[boolean, string]>);

const App = () => {
    const [fs, setFs] = useState(new MyFileSystem());
    const [targetFile, setTargetFile] = useState("");

    const [msg, showMsg] = useState<[string, string]>(["", ""]);

    const [wsLoading, setWSLoading] = useState(false);
    const [workspace, showWorkSpaceMenu] = useState<[boolean, string]>([false, ""]);

    const [system, setSystem] = useState({ units: [] });

    const parse = () => {
        const parser = Parser.new();
        try {
            fs.readAll().map(f => parser.parse(f.name, f.body) );
            setSystem(parser.check());
        } catch (err) {
            showMsg(["error", "解析エラー : "+err+""]);
            return;
        }
        showMsg(["success", "解析OK"]);
    };

    const loadWorkspace = (workspace: string) => {
        setWSLoading(true);
        axios
            .get("/editor/back/workspace/"+workspace)
            .then(response => (async () => {
                const _fs = new MyFileSystem();
                const files = response.data;
                for(var idx in files) {
                    const f = await axios.get("/editor/back/workspace/"+files[idx]);
                    _fs.mkfile(files[idx].replace(workspace, ""), f.data);
                }
                setFs(_fs);
                setTargetFile(files[0].replace(workspace, ""));
                setWSLoading(false);
            })())
            .catch(() => showMsg(["error", "指定されたワークスペースは存在しません"]));
    };

    useEffect(() => {
        init();

        const _fs = new MyFileSystem();
        _fs.mkfile("/design.def", "unit design;");
        setFs(_fs);
        setTargetFile("/design.def");
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
                onWorkspaceMenuOpen={ () => showWorkSpaceMenu([true, workspace[1]]) }
            />
            <Box
                sx={{
                    display: "flex",
                    flexDirection: "row",
                    width: "100%",
                    height: "100%"
                }}
            >
                <Backdrop
                    open={ wsLoading }
                    sx={{
                        color: "#fff",
                        zIndex: (theme) => theme.zIndex.drawer + 1
                    }}
                >
                    <CircularProgress color="inherit" />
                </Backdrop>
                <FSContext.Provider value={ fs }>
                    <TargetFileContext.Provider value={[ targetFile, setTargetFile ]}>
                        <FileExplorer width="15vw"/>
                        <Editor onSave={ parse }/>
                    </TargetFileContext.Provider>
                </FSContext.Provider>
            </Box>
            <ToolViewer
                width={ window.innerWidth * 0.4 }
                system={ system }
            />
            <MsgContext.Provider value={[ msg, showMsg ]}>
                <MsgViewer/>
            </MsgContext.Provider>
            <WorkspaceContext.Provider value={[ workspace, showWorkSpaceMenu ]}>
                <WorkspaceMenu
                    onWorkspaceOpen={ loadWorkspace }
                    onWorkSpaceCreate={ () => console.log("create") }
                />
            </WorkspaceContext.Provider>
        </Box>
    );
};

export default App;
