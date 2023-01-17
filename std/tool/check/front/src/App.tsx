import { useEffect, useState, createContext } from "react";

import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select, { SelectChangeEvent } from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import Paper from "@mui/material/Paper";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";

import init, { flistup, trace } from "sysdc_tool_check";

export const WasmContext = createContext(false);

type FEntry = [string, string]

const App = () => {
    const [wasmOk, setWasmOk] = useState(false);
    const [system, setSystem] = useState({});

    const [fEntries, setFEntries] = useState<FEntry[]>([]);
    const [traceTarget, setTraceTarget] = useState<string>("");
    const [traceResult, setTraceResult] = useState<{}>({});

    window.addEventListener("message", (e: MessageEvent) => {
        setSystem(JSON.parse(e.data))
    });

    const createFEntryList = (fEntries: FEntry[]) => {
        return fEntries.map((fEntry) => {
            return (
                <MenuItem value={fEntry[1]}>
                    ({ fEntry[0] }) { fEntry[1] }
                </MenuItem>
            );
        });
    };

    useEffect(() => {
        init().then(() => setWasmOk(true));
    }, []);

    useEffect(() => {
        if (wasmOk) {
            setFEntries(flistup(system));
        }
    }, [wasmOk, system]);

    useEffect(() => {
        if (wasmOk) {
            setTraceResult(trace(system, traceTarget));
        }
    }, [traceTarget]);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vh",
                overflowX: "hidden"
            }}
        >
            <FormControl
                variant="standard"
                sx={{
                    width: "100%",
                    margin: "10px 15px 0px 15px",
                    boxSizing: "border-box",
                    WebkitBoxSizing: "border-box",
                }}
            >
                <InputLabel id="func-selector">
                    追跡対象関数
                </InputLabel>
                <Select
                    labelId="func-selector"
                    label="追跡対象関数"
                    onChange={(e: SelectChangeEvent) => { setTraceTarget(e.target.value) }}
                >
                    {[ ...createFEntryList(fEntries) ]}
                </Select>
            </FormControl>
            { JSON.stringify(traceResult) }
            {/* <List>
                <ListItem>
                    <Paper
                        elevation={3}
                        sx={{
                            width: "100%",
                            padding: "10px 5px 10px 5px"
                        }}
                    >
                        <h2>てすと;wasmOk;wasmOk</h2>
                        ああｆさｄｊｋふぁｄ；じゃｄｓ
                    </Paper>
                </ListItem>
                <ListItem>
                    <Paper
                        elevation={3}
                        sx={{
                            width: "100%",
                            padding: "10px 5px 10px 5px"
                        }}
                    >
                        <h2>てすと;wasmOk;wasmOk</h2>
                        ああｆさｄｊｋふぁｄ；じゃｄｓ
                    </Paper>
                </ListItem>
                <ListItem>
                    <Paper
                        elevation={3}
                        sx={{
                            width: "100%",
                            padding: "10px 5px 10px 5px"
                        }}
                    >
                        <h2>てすと;wasmOk;wasmOk</h2>
                        ああｆさｄｊｋふぁｄ；じゃｄｓ
                    </Paper>
                </ListItem>
            </List> */}
        </div>
    );
}

export default App;
