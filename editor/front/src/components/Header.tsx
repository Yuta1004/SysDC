import React from "react";

import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import SettingsIcon from "@mui/icons-material/Settings";

interface HeaderProps {
    onParseClick: () => void
}

const Header = (props: HeaderProps) => {
    return (
        <AppBar
            position="static"
            color="default"
            sx={{ zIndex: 9999 }}
        >
            <Toolbar>
                <Typography
                    variant="h6"
                    component="h6"
                    sx={{ flexGrow: 1 }}
                >
                    SysDC
                </Typography>
                <Stack
                    direction="row"
                    spacing={2}
                >
                    <Button
                        variant="outlined"
                        color="success"
                        startIcon={ <PlayArrowIcon/> }
                        onClick={ props.onParseClick }
                    >
                        実行
                    </Button>
                    {/* <Button
                        variant="outlined"
                        color="info"
                        startIcon={ <SettingsIcon/> }
                    >
                        設定
                    </Button> */}
                </Stack>
            </Toolbar>
        </AppBar>
    );
};

export default Header;
