import { Handle, Position } from "react-flow-renderer";

import styles from "../../style/custom.module.css";

export const CUSTOM_NODE_TYPES = {
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
}

export default CUSTOM_NODE_TYPES;

export function UnitNode({ data }) {
    return (
        <div className={styles.Unit}>
            <h1>{data.name.name}</h1>
        </div>
    );
}

export function ModuleNode({ data }) {
    return (
        <div className={styles.Module}>
            <h1>{data.name.name}</h1>
        </div>
    );
}

export function FunctionNode({ data }) {
    return (
        <div className={styles.Function}>
            <Handle type="target" position={Position.Top}/>
            <h1>{data.name.name}</h1>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function ProcedureNode({ data }) {
    return (
        <div className={styles.Procedure}>
            <Handle type="target" position={Position.Top}/>
            <h1>{data.name.name}</h1>
        </div>
    );
}

export function ArgumentNode({ data }) {
    return (
        <div>
            Argument
            <div className={styles.Argument}>
                <div className={styles.FixedHandle}>
                    <Handle type="target" position={Position.Top}/>
                </div>
                <p className={styles.Name}>{data.name.name}</p>
                <br/>
                <p className={styles.Type}>({data.type.kind === "Data" ? data.type.refs.name : data.type.kind})</p>
                <div className={styles.FixedHandle}>
                    <Handle type="source" position={Position.Bottom}/>
                </div>
            </div>
        </div>
    );
}

export function VarNode({ data }) {
    return (
        <div className={styles.Var}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Name}>{data.name.name}</p>
            <br/>
            <p className={styles.Type}>({data.type.kind === "Data" ? data.type.refs.name : data.type.kind})</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function DeadVarNode({ data }) {
    return (
        <div className={styles.DeadVar}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Name}>{data.name.name}</p>
            <br/>
            <p className={styles.Type}>({data.type.kind === "Data" ? data.type.refs.name : data.type.kind})</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function ReturnVarNode({ data }) {
    return (
        <div>
            Return
            <div className={styles.ReturnVar}>
                <div className={styles.FixedHandle}>
                    <Handle type="target" position={Position.Top}/>
                </div>
                <p className={styles.Name}>{data.name.name}</p>
                <br/>
                <p className={styles.Type}>({data.type.kind === "Data" ? data.type.refs.name : data.type.kind})</p>
                <div className={styles.FixedHandle}>
                    <Handle type="source" position={Position.Bottom}/>
                </div>
            </div>
        </div>
    );
}

export function AffectOuterNode({ data }) {
    return (
        <div className={styles.AffectOuter}>
            <Handle className={styles.Hidden} type="target" position={Position.Top}/>
            Affect
        </div>
    );
}

export function AffectInnerNode({ data }) {
    return (
        <div className={styles.AffectInner}>
            <Handle type="source" position={Position.Right}/>
            Affect
        </div>
    );
}

export function SpawnOuterNode({ data }) {
    return (
        <div className={styles.SpawnOuter}>
            <Handle className={styles.Hidden} type="target" position={Position.Top}/>
            Spawn
            <Handle className={styles.Hidden} type="source" position={Position.Bottom}/>
        </div>
    );
}

export function SpawnInnerNode({ data }) {
    return (
        <div className={styles.SpawnInner}>
            <Handle type="target" position={Position.Right}/>
            Spawn
            <Handle type="source" position={Position.Right}/>
        </div>
    );
}
