import { useEffect, useState } from "react";

import Paper from "@mui/material/Paper";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import WarningAmberIcon from "@mui/icons-material/WarningAmber";
import InfoOutlinedIcon from "@mui/icons-material/InfoOutlined";
import CheckIcon from "@mui/icons-material/Check";

import init from "sysdc_tool_eval";

const App = () => {
    const [advice, setAdvice] = useState<Map<[number, String], String[]>>(new Map());

    const entrypoint = (event: MessageEvent) => {
        // setSystem(event.data);

        const _advice = new Map();
        _advice.set([1, "重複している可能性がある関数"], ["a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "b", "c"]);
        _advice.set([1, "分割できる可能性がある関数"], ["d", "e", "f"]);
        _advice.set([0, "その他"], ["g", "h", "i"]);
        setAdvice(_advice);
    };
    window.addEventListener("message", entrypoint);

    const makeAdviceElems = (advice: Map<[number, String], String[]>) => {
        const icons = [
            <InfoOutlinedIcon
                color="primary"
                sx={{ "verticalAlign": "top" }}
            />,
            <WarningAmberIcon
                color="warning"
                sx={{ "verticalAlign": "top" }}
            />
        ];

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

        return Array.from(advice).map(([[level, title], messages]) => {
            return (
                <Paper
                    elevation={3}
                    sx={{
                        margin: "10px",
                        padding: "10px 5px 10px 5px"
                    }}
                >
                    <h2 style={{ "margin": 0 }}>
                        { icons[level] }
                        {[ title ]}
                    </h2>
                    <List>
                        { makeMsgElems(messages) }
                    </List>
                </Paper>
            );
        });
    };

    useEffect(() => {
        init()
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
            {[ ...makeAdviceElems(advice) ]}
        </div>
    );
}

export default App;
