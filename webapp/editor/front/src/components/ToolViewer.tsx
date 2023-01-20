import { useEffect, useState, useRef, SyntheticEvent } from "react";

import Drawer from "@mui/material/Drawer";
import Toolbar from "@mui/material/Toolbar";
import Stack from "@mui/material/Stack";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import IconButton from "@mui/material/IconButton";
import RefleshOutlinedIcon from "@mui/icons-material/RefreshOutlined";
import DragHandleIcon from "@mui/icons-material/DragHandle";

interface ToolViewerProps {
    width: number,
    system: {}
}

const ToolViewer = (props: ToolViewerProps) => {
    const [viewingTool, setViewingTool] = useState("");
    const [tools, setTools] = useState<Map<string, string>>(new Map());

    const tiframe = useRef<HTMLIFrameElement>(null);

    const [drawerWidth, setDrawerWidth] = useState(props.width);
    const [mouseOnDrawer, setMouseOnDrawer] = useState(false);
    const [drawerDragging, setDrawerDragging] = useState(false);
    const drawerDraggingR = useRef(false);
    drawerDraggingR.current = drawerDragging;

    window.addEventListener("mouseup", () => setDrawerDragging(false));

    useEffect(() => {
        if (tiframe.current !== null && tiframe.current.contentWindow !== null) {
            const iwindow = tiframe.current.contentWindow;
            tiframe.current.onload = () => iwindow.postMessage(props.system);
            iwindow.postMessage(props.system);
        }
    }, [tiframe.current, viewingTool, props.system]);

    useEffect(() => {
        const _tools = new Map();
        _tools.set("std@debug v1.0.1", "/std/debug/1.0.1");
        _tools.set("std@json v1.0.1",  "/std/json/1.0.1");
        _tools.set("std@view v0.2.1",  "/std/view/0.2.1");
        _tools.set("std@eval v0.1.0",  "/std/eval/0.1.0");
        _tools.set("std@check v0.1.0", "/std/check/0.1.0");
        setTools(_tools);
    }, []);

    return (
        <Drawer
            id="test"
            variant="persistent"
            anchor="right"
            open={true}
            hideBackdrop={true}
            sx={{ [`& .MuiDrawer-paper`]: { minWidth: drawerWidth } }}
            onMouseMove={(e: SyntheticEvent<HTMLDivElement, MouseEvent>) => {
                if (drawerDraggingR.current) {
                    setDrawerWidth((prev) => prev - e.nativeEvent.movementX);
                }
            }}
        >
            <Toolbar/>
            <Stack direction="row">
                <DragHandleIcon
                    sx={{
                        margin: "auto",
                        paddingLeft: "10px"
                    }}
                    onMouseOver={ () => setMouseOnDrawer(true) }
                    onMouseOut={ () => setMouseOnDrawer(false) }
                    onMouseDown={ () => setDrawerDragging(mouseOnDrawer && true) }
                />
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
                            value="default">
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
                <IconButton onClick={ () => tiframe.current?.contentWindow?.location.reload() }>
                    <RefleshOutlinedIcon/>
                </IconButton>
            </Stack>
            <iframe
                ref={ tiframe }
                width="100%"
                height="100%"
                key={ viewingTool }
                src={ "/tool/delivery" + tools?.get(viewingTool) }
            />
        </Drawer>
    );
};

export default ToolViewer;
