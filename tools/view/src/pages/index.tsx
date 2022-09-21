import React, { useEffect, useMemo } from "react";
import ReactFlow, { Background, MiniMap, Controls, useNodesState, useEdgesState } from "react-flow-renderer";
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import IconButton from '@mui/material/IconButton';
import MenuIcon from '@mui/icons-material/Menu';
import RefreshIcon from "@mui/icons-material/Refresh";
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

    useEffect(() => {
        invoke("gen_flow").then(([nodes, edges]) => {
            layout(nodes, edges);
            Array.isArray(nodes) && setNodes(nodes);
            Array.isArray(edges) && setEdges(edges);
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
            >
                <Toolbar>
                    <IconButton
                        size="large"
                        edge="start"
                        color="inherit"
                        aria-label="menu"
                        sx={{ mr: 2 }}
                    >
                        <MenuIcon/>
                    </IconButton>
                    <Typography variant="h6" component="h6" sx={{ flexGrow: 1 }}>
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
            </AppBar>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                onNodesChange={onNodesChange}
                onEdgesChange={onEdgesChange}
                nodeTypes={customNodeTypes}
                defaultEdgeOptions={{ zIndex: 9999 }}
                minZoom={0}
                fitView
            >
                <Background gap={24} size={1.5} color="#0006"/>
                <MiniMap/>
                <Controls/>
            </ReactFlow>
        </div>
    );
}

export default App;
