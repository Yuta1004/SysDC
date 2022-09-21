import React, { useState, useEffect, useMemo } from "react";
import ReactFlow, { Background, MiniMap, Controls, useNodesState, useEdgesState } from "react-flow-renderer";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Drawer from "@mui/material/Drawer";
import List from '@mui/material/List';
import ListSubheader from "@mui/material/ListSubheader";
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Collapse from "@mui/material/Collapse";
import Typography from "@mui/material/Typography";
import IconButton from "@mui/material/IconButton";
import LinearProgress from "@mui/material/LinearProgress";
import MenuIcon from "@mui/icons-material/Menu";
import RefreshIcon from "@mui/icons-material/Refresh";
import ExpandLess from "@mui/icons-material/ExpandLess";
import ExpandMore from "@mui/icons-material/ExpandMore";
import Inventory from "@mui/icons-material/Inventory";
import ArrowDownward from "@mui/icons-material/ArrowDownward";
import ImportExport from "@mui/icons-material/ImportExport";
import { invoke } from "@tauri-apps/api/tauri";

import layout from "../flow/layout";
import convert from "../sysdc_core/convert";
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
    const [unitListItems, setUnitListItems] = useState([]);
    const [expandUnitList, setExpandUnitList] = useState({});

    useEffect(() => {
        invoke("get_system").then(_system => {
            const system = (typeof _system == "object" && convert(_system));
            const _unitListItems = system.units.map(unit => {
                const dataItems = unit.data.map(data =>
                    <ListItemButton key={data.name.name} sx={{ pl: 4 }}>
                        <ListItemIcon>
                            <Inventory/>
                        </ListItemIcon>
                        <ListItemText primary={data.name.name}/>
                    </ListItemButton>
                );
                const modItems = unit.modules.map(mod => {
                    const funcItems = mod.functions.map(func =>
                        <ListItemButton key={func.name.name} sx={{ pl: 8 }}>
                            <ListItemIcon>
                                {func.return[1] === "void" ? <ArrowDownward/> : <ImportExport/> }
                            </ListItemIcon>
                            <ListItemText primary={func.name.name}/>
                        </ListItemButton>
                    );
                    return (<>
                        <ListItemButton
                            key={mod.name.name}
                            sx={{ pl: 4 }}
                            onClick={() => {
                                const _expandUnitList = expandUnitList;
                                _expandUnitList[mod.name.fname] = !expandUnitList[mod.name.fname];
                                setExpandUnitList(JSON.parse(JSON.stringify(_expandUnitList)));
                            }}
                        >
                            <ListItemText primary={mod.name.name}/>
                            {expandUnitList[mod.name.fname] ? <ExpandLess/> : <ExpandMore/>}
                        </ListItemButton>
                        <Collapse
                            in={expandUnitList[mod.name.namespace+"."+mod.name.name]}
                            timeout="auto"
                            unmountOnExit
                        >
                            <List component="div" disablePadding>{funcItems}</List>
                        </Collapse>
                    </>);
                });
                return (<>
                    <ListItemButton
                        key={unit.name.namespace+"."+unit.name.name}
                        onClick={() => {
                            const _expandUnitList = expandUnitList;
                            _expandUnitList[unit.name.fname] = !expandUnitList[unit.name.fname];
                            setExpandUnitList(JSON.parse(JSON.stringify(_expandUnitList)));
                        }}
                    >
                        <ListItemText primary={unit.name.namespace+"."+unit.name.name}/>
                        {expandUnitList[unit.name.fname] ? <ExpandLess/> : <ExpandMore/>}
                    </ListItemButton>
                    <Collapse
                        in={expandUnitList[unit.name.namespace+"."+unit.name.name]}
                        timeout="auto"
                        unmountOnExit
                    >
                        <List component="div" disablePadding>{[...dataItems, ...modItems]}</List>
                    </Collapse>
                </>);
            });
            setUnitListItems(_unitListItems);
        });
    }, [expandUnitList]);

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
                    padding: "25px",
                    flexShrink: 0,
                    [`& .MuiDrawer-paper`]: { minWidth: "15%", boxSizing: 'border-box' }
                }}
            >
                <Toolbar/>
                <List subheader={
                    <ListSubheader component="div" id="nested-list-subheader">
                        構成
                    </ListSubheader>
                }>
                    {unitListItems}
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
