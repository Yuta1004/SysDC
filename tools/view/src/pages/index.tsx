import { useEffect, useState } from "react";
import { Node, Edge } from "react-flow-renderer";
import { invoke } from "@tauri-apps/api/tauri";

import FlowComponent from "../components/flow";

function App() {
    const [nodes, setNodes] = useState<Node<any>[]>([]);
    const [edges, setEdges] = useState<Edge<any>[]>([]);

    useEffect(() => {
        invoke("get_flow").then(([nodes, edges]) => {
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
            <FlowComponent
                nodes={nodes}
                edges={edges}
            />
        </div>
    );
}

export default App;
