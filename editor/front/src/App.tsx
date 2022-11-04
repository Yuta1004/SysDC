import { useEffect, useState } from "react";

import Header from "./components/Header";
import FileExplorer from "./components/FileExplorer";
import Editor from "./components/Editor";
import MyFileSystem from "./filesystem/MyFileSystem";

const App = () => {
    const [fs, _setFs] = useState(new MyFileSystem());
    const [targetFile, setTargetFile] = useState("/design.def");

    useEffect(() => {
        fs.mkfile("/design.def", "unit design;");
    });

    return (
        <div
            style={{
                display: "flex",
                flexDirection: "column",
                width: "100vw",
                height: "100vh"
            }} 
        >
            <Header
                style={{
                    flex: 1
                }}
            />
            <div
                style={{
                    display: "flex",
                    flexDirection: "row",
                    width: "100%",
                    height: "100%"
                }}
            >
                <FileExplorer
                    fs={ fs }
                    style={{
                        padding: 0,
                        flex: 1,
                        minWidth: "10%"
                    }}
                    onSelect={ path => setTargetFile(path) }
                />
                <Editor
                    fs={ fs }
                    targetFile={ targetFile }
                    style={{
                        width: "100%",
                        height: "100%"
                    }} 
                />
            </div>
        </div>
    )
};

export default App;
