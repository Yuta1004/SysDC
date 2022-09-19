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
            <h1>Function</h1>
        </div>
    );
}

export function VarNode({ data }) {
    return (
        <div className={styles.Var}>
            <Handle type="target" position={Position.Top}/>
            <p className={styles.Var_Name}>name</p>
            <br/>
            <p className={styles.Var_Type}>(type)</p>
            <Handle type="source" position={Position.Bottom}/>
        </div>
    );
}