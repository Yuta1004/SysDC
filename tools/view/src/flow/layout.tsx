import { Node, Edge } from "react-flow-renderer";
import dagre from "dagre";

export default layout;

const MARGIN_TOP_BOTTOM = 200;
const MARGIN_LEFT_RIGHT = 200;
const DEFAULT_NODE_WIDTH = 250;
const DEFAULT_NODE_HEIGHT = 100;

export function layout(nodes: Node<any>[], edges: Edge<any>[]) {
    const isUnit = node =>
        node.type === "Unit";
    const isModule = (node, pnode) =>
        node.type === "Module" && node.parentNode === pnode.id;
    const isFunction = (node, pnode) =>
        node.type === "Function" && node.parentNode === pnode.id;
    const isFunctionChild = (node, pnode) =>
        ["Argument", "Var", "ReturnVar"].includes(node.type) && node.parentNode === pnode.id;

    const unodes = nodes.filter(isUnit).map(unode => {
        const mnodes = nodes.filter(node => isModule(node, unode)).map(mnode => {
            const fnodes = nodes.filter(node => isFunction(node, mnode)).map(fnode => {
                const vnodes = nodes.filter(node => isFunctionChild(node, fnode));
                autoLayout(vnodes, edges);
                fnode.style = getFlowSize(vnodes);
                return fnode;
            });
            autoLayout(fnodes, edges);
            mnode.style = getFlowSize(fnodes);
            return mnode;
        });
        autoLayout(mnodes, edges);
        unode.style = getFlowSize(mnodes);
        return unode;
    });
    autoLayout(unodes, edges);

    nodes.forEach(node => {
        node.position.x += MARGIN_LEFT_RIGHT/2;
        node.position.y += MARGIN_TOP_BOTTOM/2;
    });
}

function autoLayout(nodes: Node<any>[], edges: Edge<any>[]): Node<any>[] {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setGraph({ rankdir: "TB" });
    dagreGraph.setDefaultEdgeLabel(() => ({}));

    nodes.forEach(node => {
        if (node.style == undefined) {
            dagreGraph.setNode(node.id, { width: DEFAULT_NODE_WIDTH, height: DEFAULT_NODE_HEIGHT });
        } else {
            dagreGraph.setNode(node.id, { width: node.style.width, height: node.style.height });
        }
    });
    edges.forEach(edge => {
        dagreGraph.setEdge(edge.source, edge.target);
    });

    dagre.layout(dagreGraph);

    return nodes.map(node => {
        const nodeWithPos = dagreGraph.node(node.id);
        const width = node.style == undefined ? DEFAULT_NODE_WIDTH : +node.style.width;
        const height = node.style == undefined ? DEFAULT_NODE_HEIGHT : +node.style.height;
        node.position = {
            x: nodeWithPos.x-width/2,
            y: nodeWithPos.y-height/2
        };
        return node;
    });
}

function getFlowSize(nodes: Node<any>[]): { width: number, height: number } {
    let cornerTL: [number, number] = [999999999, 999999999];
    let cornerBR: [number, number] = [0, 0];
    nodes.forEach(node => {
        const width = node.style == undefined ? DEFAULT_NODE_WIDTH : +node.style.width;
        const height = node.style == undefined ? DEFAULT_NODE_HEIGHT : +node.style.height;
        cornerTL = [Math.min(node.position.x, cornerTL[0]), Math.min(node.position.y, cornerTL[1])];
        cornerBR = [Math.max(node.position.x+width, cornerBR[0]), Math.max(node.position.y+height, cornerBR[1])];
    });
    nodes.forEach(node => {
        node.position.x -= cornerTL[0];
        node.position.y -= cornerTL[1];
    });
    return {
        width: Math.max(cornerBR[0]-cornerTL[0], 200) + MARGIN_LEFT_RIGHT,
        height: Math.max(cornerBR[1]-cornerTL[1], 50) + MARGIN_TOP_BOTTOM
    };
}
