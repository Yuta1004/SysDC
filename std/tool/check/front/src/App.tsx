import { useEffect, useState, useReducer } from "react";

import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import Select, { SelectChangeEvent } from "@mui/material/Select";
import MenuItem from "@mui/material/MenuItem";
import Paper from "@mui/material/Paper";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ButtonGroup from "@mui/material/ButtonGroup";
import Button from "@mui/material/Button";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import ArrowRightAltIcon from "@mui/icons-material/ArrowRightAlt";
import SwitchAccessShortcutIcon from "@mui/icons-material/SwitchAccessShortcut";
import SwitchAccessShortcutAddIcon from "@mui/icons-material/SwitchAccessShortcutAdd";

import init, { flistup, trace, trace_var } from "sysdc_tool_check";

type FEntry = [string, string]
type TResult = [string, {}]

const App = () => {
    const [, forceUpdate] = useReducer(x => ++x, 0);

    const [wasmOk, setWasmOk] = useState(false);
    const [system, setSystem] = useState({});

    const [fEntries, setFEntries] = useState<FEntry[]>([]);
    const [showingFEntry, setShowingFEntry] = useState<string>("");

    const [traceTarget, setTraceTarget] = useState<string>("");
    const [traceResult, setTraceResult] = useState<TResult[]>([]);
    const [traceResultDetail, setTraceResultDetail] = useState<Map<string, JSX.Element>>(new Map());

    window.addEventListener("message", (e: MessageEvent) => {
        setSystem(e.data)
    });

    const createFEntryList = (fEntries: FEntry[]) => {
        return fEntries.map((fEntry) => {
            return (
                <MenuItem value={ fEntry[1] }>
                    ({ fEntry[0] }) { fEntry[1] }
                </MenuItem>
            );
        });
    };

    const createTResultListSub = (result: any) => {
        const [tvname, elems] = result;
        return elems.map((elem: any) => {
            var kind: string, details: any;
            if (typeof elem === "string") {
                kind = "Return";
            } else {
                kind = Object.keys(elem)[0];
                details = Object.values(elem)[0];
            }

            switch (kind) {
                case "Return":
                    return (
                        <Accordion>
                            <AccordionSummary>
                                <InfoOutlinedIcon/>
                                <div
                                    style={{
                                        "display": "flex",
                                        "alignItems": "center"
                                    }}
                                >
                                    この変数の値は返り値として使用されます
                                </div>
                            </AccordionSummary>
                        </Accordion>
                    );

                case "Affect":
                    return (
                        <Accordion>
                            <AccordionSummary
                                expandIcon={ <ExpandMoreIcon />}
                            >
                                <ArrowRightAltIcon/>
                                <div
                                    style={{
                                        "display": "flex",
                                        "alignItems": "center"
                                    }}
                                >
                                    この変数の値を使用して関数 { details["func"] } を呼び出します
                                </div>
                            </AccordionSummary>
                            <AccordionDetails>
                                {[ ...createTResultListSub([details["arg_to"], trace_var(system, details["arg_to"])]) ]}
                            </AccordionDetails>
                        </Accordion>
                    );

                case "ModifyVarL": {
                    const vars: string[] = details["vars"];
                    const buttons: JSX.Element[] = vars.map((vname: string) => {
                        return (
                            <Button
                                sx={{ "textTransform": "none" }} 
                                onClick={() => {
                                    traceResultDetail.set(
                                        "amodify"+tvname, 
                                        <>
                                            <p><b>{ vname.split(".").pop() }</b></p>
                                            {[ ...createTResultListSub([vname, trace_var(system, vname)]) ]}
                                        </>
                                    );
                                    setTraceResultDetail(traceResultDetail);
                                    forceUpdate();
                                }}
                            >
                                { vname.split(".").pop() }
                            </Button>
                        );
                    });
                    return (
                        <Accordion>
                            <AccordionSummary
                                expandIcon={ <ExpandMoreIcon />}
                            >
                                <SwitchAccessShortcutIcon/>
                                <div
                                    style={{
                                        "display": "flex",
                                        "alignItems": "center"
                                    }}
                                >
                                    他の変数の値を使用して値を更新します
                                </div>
                            </AccordionSummary>
                            <AccordionDetails>
                                <ButtonGroup>
                                    {[ ...buttons ]}
                                </ButtonGroup>
                                { traceResultDetail.get("amodify"+tvname) }
                            </AccordionDetails>
                        </Accordion>
                    );
                }

                case "SpawnVarL": {
                    const vars: string[] = details["vars"];
                    const buttons: JSX.Element[] = vars.map((vname: string) => {
                        return (
                            <Button
                                sx={{ "textTransform": "none" }} 
                                onClick={() => {
                                    traceResultDetail.set(
                                        "aspawn"+tvname, 
                                        <>
                                            <p><b>{ vname.split(".").pop() }</b></p>
                                            {[ ...createTResultListSub([vname, trace_var(system, vname)]) ]}
                                        </>
                                    );
                                    setTraceResultDetail(traceResultDetail);
                                    forceUpdate();
                                }}
                            >
                                { vname.split(".").pop() }
                            </Button>
                        );
                    });
                    return (
                        <Accordion>
                            <AccordionSummary
                                expandIcon={ <ExpandMoreIcon />}
                            >
                                <SwitchAccessShortcutAddIcon/>
                                <div
                                    style={{
                                        "display": "flex",
                                        "alignItems": "center"
                                    }}
                                >
                                    他の変数の値を使用して作成します
                                </div>
                            </AccordionSummary>
                            <AccordionDetails>
                                <ButtonGroup>
                                    {[ ...buttons ]}
                                </ButtonGroup>
                                { traceResultDetail.get("aspawn"+tvname) }
                            </AccordionDetails>
                        </Accordion>
                    );
                }
            }
        });
    }

    const createTResultList = (traceResult: TResult[]) => {
        return traceResult.map((result) => {
            return (
                <ListItem>
                    <Paper
                        elevation={3}
                        sx={{
                            width: "100%",
                            padding: "10px 5px 10px 5px"
                        }}
                    >
                        <h2 style={{ "margin": 0 }}>
                            <span style={{
                                "background": "linear-gradient(transparent 75%, rgba(245, 66, 96, 30) 100%)",
                                "lineHeight": 1
                            }}>
                            { result[0].split(".").pop() }
                            </span>
                        </h2>
                        {[ ...createTResultListSub(result) ]}
                    </Paper>
                </ListItem>
            );
        });
    };

    useEffect(() => {
        init().then(() => setWasmOk(true));
    }, []);

    useEffect(() => {
        if (wasmOk) {
            let fentries = flistup(system);
            setFEntries(fentries);
            setShowingFEntry(fentries[0][1]);
            setTraceTarget(fentries[0][1]);
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
                    value={showingFEntry}
                    onChange={(e: SelectChangeEvent) => {
                        setTraceTarget(e.target.value)
                        setShowingFEntry(e.target.value);
                    }}
                >
                    {[ ...createFEntryList(fEntries) ]}
                </Select>
            </FormControl>
            <List>
                {[ ...createTResultList(traceResult) ]}
            </List>
        </div>
    );
}

export default App;
