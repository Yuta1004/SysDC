import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import PeopleIcon from "@mui/icons-material/People";

interface HeaderProps {
    onParseClick: () => void,
    onWorkspaceMenuOpen: () => void
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
                        解析
                    </Button>
                    <Button
                        variant="outlined"
                        color="info"
                        startIcon={ <PeopleIcon/> }
                        onClick={ props.onWorkspaceMenuOpen }
                    >
                        ワークスペース
                    </Button>
                </Stack>
            </Toolbar>
        </AppBar>
    );
};

export default Header;
