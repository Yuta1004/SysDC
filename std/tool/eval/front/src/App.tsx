import { useEffect, useState } from "react";

import Paper from "@mui/material/Paper";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import WarningAmberIcon from "@mui/icons-material/WarningAmber";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import CheckIcon from "@mui/icons-material/Check";

import init, { test } from "sysdc_tool_eval";

interface Advice {
    level: String,
    title: String,
    messages: String[]
}

const App = () => {
    const [wasmOk, setWasmOk] = useState<Boolean>(false); 
    const [system, setSystem] = useState<{}>({});
    const [advice, setAdvice] = useState<Advice[]>([]);

    window.addEventListener("message", (e: MessageEvent) => setSystem(e.data));

    const makeAdviceElems = (advice: Advice[]) => {
        const icons: Map<String, JSX.Element> = new Map([
            [
                "Info",
                <InfoOutlinedIcon
                    color="primary"
                    sx={{ "verticalAlign": "top" }}
                />
            ],
            [
                "Warning",
                <WarningAmberIcon
                    color="warning"
                    sx={{ "verticalAlign": "top" }}
                />
            ]
        ]);

        const makeMsgElems = (messages: String[]) => {
            return messages.map((msg) => {
                return (
                    <ListItem>
                        <CheckIcon sx={{ "margin": "5px" }}/>
                        { msg }
                    </ListItem>
                );
            });
        };

        return advice.map((adv) => {
            return (
                <Paper
                    elevation={3}
                    sx={{
                        margin: "10px",
                        padding: "10px 5px 10px 5px"
                    }}
                >
                    <h2 style={{ "margin": 0 }}>
                        { icons.get(adv.level) }
                        {[ adv.title ]}
                    </h2>
                    <List>
                        { makeMsgElems(adv.messages) }
                    </List>
                </Paper>
            );
        });
    };

    useEffect(() => {
        init().then(() => setWasmOk(true));
    }, []);

    useEffect(() => {
        if (wasmOk) {
            setAdvice(test()); 
        }
    }, [wasmOk, system]);

    return (
        <div
            className="container"
            style={{
                width: "100vw",
                height: "100vh",
                overflowX: "hidden"
            }}
        >
            {[ ...makeAdviceElems(advice) ]}
        </div>
    );
}

export default App;
