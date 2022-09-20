import { Handle, Position } from "react-flow-renderer";

import styles from "../style/custom.module.css";

export function UnitNode({ data }) {
    return (
        <div className={styles.Unit}>
            <h1>Unit</h1>
        </div>
    );
}

export function ModuleNode({ data }) {
    return (
        <div className={styles.Module}>
            <h1>Module</h1>
        </div>
    );
}

export function FunctionNode({ data }) {
    return (
        <div className={styles.Function}>
            <Handle type="target" position={Position.Top}/>
            <h1>Function</h1>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function ProcedureNode({ data }) {
    return (
        <div className={styles.Procedure}>
            <Handle type="target" position={Position.Top}/>
            <h1>Procedure</h1>
        </div>
    );
}

export function ArgumentNode({ data }) {
    return (
        <div className={styles.Argument}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Name}>name</p>
            <br/>
            <p className={styles.Type}>(type)</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function VarNode({ data }) {
    return (
        <div className={styles.Var}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Name}>name</p>
            <br/>
            <p className={styles.Type}>(type)</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function DeadVarNode({ data }) {
    return (
        <div className={styles.DeadVar}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Name}>name</p>
            <br/>
            <p className={styles.Type}>(type)</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function ReturnVarNode({ data }) {
    return (
        <div className={styles.ReturnVar}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Name}>name</p>
            <br/>
            <p className={styles.Type}>(type)</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}

export function AffectOuterNode({ data }) {
    return (
        <div className={styles.AffectOuter}>
            <Handle type="target" position={Position.Top}/>
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
            <Handle type="target" position={Position.Top}/>
            Spawn
            <Handle type="source" position={Position.Bottom}/>
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
