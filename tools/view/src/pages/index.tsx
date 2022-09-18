import { useEffect, useState } from "react";
import ReactFlow from "react-flow-renderer";
import { invoke } from "@tauri-apps/api/tauri";

import { convert } from "../sysdc_core/convert";

function App() {
    const [system, setSystem] = useState({ units: [] });
    const [nodes, setNodes] = useState([]);
    const [edges, setEdges] = useState([]);

    useEffect(() => {
        Promise.all([
            invoke("get_system"),
            invoke("get_nodes"),
            invoke("get_edges")
        ]).then(([system, nodes, edges]) => {
            if (typeof system == "object") {
                setSystem(convert(system));
            }
            if (Array.isArray(nodes)) {
                nodes.forEach(node => node["position"] = {x: Math.random()*120, y: Math.random()*120});
                console.log(nodes);
                setNodes(nodes);
            }
            if (Array.isArray(edges)) {
                setEdges(edges);
            }
        });
    }, []);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vw"
            }}
        >
            <h1>SysDC</h1>
            <p>{ JSON.stringify(system) }</p>
            <hr/>
            <ReactFlow
                nodes={nodes}
                edges={edges}
                fitView
            />
        </div>
    );
}

export default App;
