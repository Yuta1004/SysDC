import { useEffect, useState, createContext } from "react";

import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import Paper from "@mui/material/Paper";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";

import init from "sysdc_tool_check";

export const WasmContext = createContext(false);

interface FEntry {
    kind: string,
    name: string
}

const App = () => {
    const [wasmOk, setWasmOk] = useState(false);
    const [system, setSystem] = useState("{}");

    const [fEntries, setFEntries] = useState<FEntry[]>([]);

    window.addEventListener("message", (e: MessageEvent) => {
        setSystem(e.data);

        setFEntries([
            { kind: "Proc", name: "aaa.aaa.bbb.ccc" },
            { kind: "Func", name: "aaa.aaa.bbb.ddd" },
            { kind: "Func", name: "aaa.aaa.bbb.eee" },
        ]);
    });

    const createFEntryList = (fEntries: FEntry[]) => {
        return fEntries.map((fEntry) => {
            return (
                <MenuItem value={fEntry.name}>
                    ({ fEntry.kind }) { fEntry.name }
                </MenuItem>
            );
        });
    };

    useEffect(() => {
        init().then(() => setWasmOk(true));
    }, []);

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
                >
                    {[ ...createFEntryList(fEntries) ]}
                </Select>
            </FormControl>
            <List>
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
            </List>
        </div>
    );
}

export default App;
