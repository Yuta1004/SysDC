import React, { useState, useEffect, useMemo } from "react";
import ReactFlow, { Background, MiniMap, Controls, useNodesState, useEdgesState } from "react-flow-renderer";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Drawer from "@mui/material/Drawer";
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListSubheader from "@mui/material/ListSubheader";
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Collapse from "@mui/material/Collapse";
import FormGroup from "@mui/material/FormGroup";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import LinearProgress from "@mui/material/LinearProgress";
import MenuIcon from "@mui/icons-material/Menu";
import RefreshIcon from "@mui/icons-material/Refresh";
import ExpandLess from "@mui/icons-material/ExpandLess";
import { invoke } from "@tauri-apps/api/tauri";

import layout from "../flow/layout";
import {
    UnitNode,
    ModuleNode,
    FunctionNode,
    ProcedureNode,
    ArgumentNode,
    VarNode,
    DeadVarNode,
    ReturnVarNode,
    AffectOuterNode,
    AffectInnerNode,
    SpawnOuterNode,
    SpawnInnerNode
} from "../flow/custom";

function App() {
    /* ReactFlowで扱うState */
    const [nodes, setNodes, onNodesChange] = useNodesState([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState([]);
    const customNodeTypes = useMemo(() => ({
        Unit: UnitNode,
        Module: ModuleNode,
        Function: FunctionNode,
        Procedure: ProcedureNode,
        Argument: ArgumentNode,
        Var: VarNode,
        DeadVar: DeadVarNode,
        ReturnVar: ReturnVarNode,
        AffectOuter: AffectOuterNode,
        AffectInner: AffectInnerNode,
        SpawnOuter: SpawnOuterNode,
        SpawnInner: SpawnInnerNode
    }), []);

    /* 描画制御周りで扱うState */
    const [generatingFlow, setGeneratingFlow] = useState(true);
    const [openDrawer, setOpenDrawer] = useState(false);

    useEffect(() => {
        invoke("gen_flow").then(([nodes, edges]) => {
            layout(nodes, edges);
            Array.isArray(nodes) && setNodes(nodes);
            Array.isArray(edges) && setEdges(edges);
            setGeneratingFlow(false)
        });
    }, []);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vh"
            }}
        >
            <AppBar
                position="fixed"
                color="default"
                sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}
            >
                <Toolbar>
                    <IconButton
                        size="large"
                        edge="start"
                        color="inherit"
                        aria-label="menu"
                        sx={{ mr: 2 }}
                        onClick={() => setOpenDrawer(!openDrawer)}
                    >
                        <MenuIcon/>
                    </IconButton>
                    <Typography
                        variant="h6"
                        component="h6"
                        sx={{ flexGrow: 1 }}
                    >
                        SysDC-View
                    </Typography>
                    <IconButton
                        size="large"
                        color="inherit"
                        onClick={() => { window.location.reload() }}
                    >
                        <RefreshIcon/>
                    </IconButton>
                </Toolbar>
                <LinearProgress style={{ display: generatingFlow ? "block" : "none" }}/>
            </AppBar>
            <Drawer
                variant="persistent"
                anchor="left"
                open={openDrawer}
                sx={{
                    minWidth: "15%",
                    flexShrink: 0,
                    [`& .MuiDrawer-paper`]: { minWidth: "15%", boxSizing: 'border-box' }
                }}
            >
                <Toolbar/>
                <List subheader={
                    <ListSubheader component="div" id="nested-list-subheader">
                        表示
                    </ListSubheader>
                }>
                    <ListItemButton>
                        <ListItemIcon>
                            <MenuIcon/>
                        </ListItemIcon>
                        <ListItemText primary="Unit"/>
                        <ExpandLess/>
                    </ListItemButton>
                    <Collapse in={true} timeout="auto" unmountOnExit>
                        <List component="div" disablePadding>
                            {[".0.test", ".0.test.A", ".0.test.B", ".0.test.B.aaa"].map((text, _) => (
                                <ListItem key={text} sx={{ pl: 4 }} disablePadding>
                                    <FormGroup>
                                        <FormControlLabel control={<Checkbox defaultChecked />} label={text} />
                                    </FormGroup>
                                </ListItem>
                            ))}
                        </List>
                    </Collapse>
                </List>
            </Drawer>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                nodeTypes={customNodeTypes}
                defaultEdgeOptions={{ zIndex: 9999 }}
                minZoom={0}
                fitView
                style={{ visibility: generatingFlow ? "hidden" : "visible" }}
            >
                <Background
                    gap={24}
                    size={1.5}
                    color="#0006"
                />
                <MiniMap/>
                <Controls/>
            </ReactFlow>
        </div>
    );
}

export default App;
