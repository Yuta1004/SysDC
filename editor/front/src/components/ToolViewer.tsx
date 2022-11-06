import { useEffect, useState } from "react";

import Drawer from "@mui/material/Drawer";
import Toolbar from "@mui/material/Toolbar";
import Stack from "@mui/material/Stack";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";

const ToolViewer = () => {
    const [viewingTool, setViewingTool] = useState("");
    const [tools, setTools] = useState<Map<string, string>>();
    const [selector, setSelector] = useState<JSX.Element>();

    const loadTools = () => {
        var tools = new Map();
        tools.set("std@viewer v0.1.0", "https://sysdc.nakagamiyuta.dev");
        tools.set("std@json v0.1.0", "https://sysdc.nakagamiyuta.dev");
        tools.set("std@debug v0.1.0", "https://sysdc.nakagamiyuta.dev");

        const selector = (
            <Select
                defaultValue={ tools.keys().next().value }
            >
                {Array.from(tools).map(([name, _url]) => {
                    return (
                        <MenuItem
                            value={name}
                            onClick={ () => setViewingTool(name) }
                        >
                            {name}
                        </MenuItem>
                    );
                })}
            </Select>
        );

        setViewingTool( tools.keys().next().value );
        setTools(tools);
        setSelector(selector);
    };

    useEffect(() => {
        loadTools();
    }, []);

    return (
        <Drawer
            variant="persistent"
            anchor="right"
            open={true}
            hideBackdrop={true}
            sx={{
                [`& .MuiDrawer-paper`]: {
                    width: "40%",
                    minWidth: "300px"
                }
            }}
        >
            <p
                style={{
                    position: "absolute",
                    top: "30%",
                    left: "-20%",
                }}
            >
                test
            </p>
            <Toolbar/>
            <Stack
                direction="row"
            >
                <FormControl
                    size="small"
                    style={{
                        padding: "10px",
                        flexGrow: 1
                    }} 
                >
                    { selector }
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
                width="100%"
                height="100%"
                key={ viewingTool }
                src={ tools?.get(viewingTool) }
            />
        </Drawer>
    );
};

export default ToolViewer;
