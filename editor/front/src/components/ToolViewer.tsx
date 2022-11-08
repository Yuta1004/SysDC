import { useEffect, useState, useRef } from "react";

import Drawer from "@mui/material/Drawer";
import Toolbar from "@mui/material/Toolbar";
import Stack from "@mui/material/Stack";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";

interface ToolViewerProps {
    width: string,
    system: string
}

const ToolViewer = (props: ToolViewerProps) => {
    const [viewingTool, setViewingTool] = useState("");
    const [tools, setTools] = useState<Map<string, string>>(new Map());

    const tiframe = useRef<HTMLIFrameElement>(null);

    useEffect(() => {
        if (tiframe.current !== null && tiframe.current.contentWindow !== null) {
            const iwindow = tiframe.current.contentWindow;
            tiframe.current.onload = () => iwindow.postMessage(props.system);
            iwindow.postMessage(props.system);
        }
    }, [viewingTool, props.system]);

    return (
        <Drawer
            variant="persistent"
            anchor="right"
            open={true}
            hideBackdrop={true}
            sx={{ [`& .MuiDrawer-paper`]: { minWidth: props.width } }}
        >
            <Toolbar/>
            <Stack direction="row">
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
                {/* <IconButton
                    style={{
                        padding: "10px"
                    }} 
                >
                    <RefleshOutlinedIcon/> */}
                {/* </IconButton> */}
            </Stack>
            <iframe
                ref={ tiframe }
                width="100%"
                height="100%"
                key={ viewingTool }
                src={ tools?.get(viewingTool) }
            />
        </Drawer>
    );
};

export default ToolViewer;
