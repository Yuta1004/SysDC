import { useState, useEffect, useRef } from "react";

import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";

const App = () => {
    const [viewingTool, setViewingTool] = useState("");
    const [tools, setTools] = useState<Map<string, string>>(new Map());

    const system = { units: [] }

    useEffect(() => {
        const _tools = new Map();
        _tools.set("std@debug v1.0.1", "/debug/index.html");
        _tools.set("std@json v1.0.1",  "/json/index.html");
        _tools.set("std@view v0.2.1",  "/view/index.html");
        _tools.set("std@eval v0.1.0",  "/eval/index.html");
        _tools.set("std@check v0.1.0", "/check/index.html");
        setTools(_tools);
    }, []);

    return (
        <div
            style={{
                display: "flex",
                flexDirection: "column",
                width: "100vw",
                height: "100vh",
                boxSizing: "border-box",
                WebkitBoxSizing: "border-box",
                overflow: "hidden"
            }} 
        >
            <Stack
                direction="row"
            >
                <Button
                    variant="outlined"
                    color="success"
                    sx={{ margin: "10px" }}
                    startIcon={ <PlayArrowIcon/> }
                    onClick={ () => {
                        const iwindow = document.getElementsByTagName("iframe")[0].contentWindow;
                        iwindow.postMessage(system, "*");
                    }}
                >
                    実行
                </Button>
                <FormControl
                    size="small"
                    sx={{
                        padding: "10px",
                        flexGrow: 1
                    }}
                >
                    <Select defaultValue="default">
                        <MenuItem
                            disabled
                            value="default"
                        >
                            <em>ツールを選択してください</em>
                        </MenuItem>
                        {Array.from(tools).map(([name, _url]) => {
                            return (
                                <MenuItem
                                    value={ name }
                                    onClick={ () => setViewingTool(name) }
                                >
                                    { name }
                                </MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
            </Stack>
            <iframe
                key={ viewingTool }
                width="100%"
                height="100%"
                src={ (() => {
                    if (viewingTool === "") {
                        return "/static/tools/debug/index.html";
                    }
                    return "/static/tools" + tools?.get(viewingTool);
                })() }
            />
        </div>
    );
}

export default App;
