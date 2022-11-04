import List from "@mui/material/List";
import ListItemButton from "@mui/material/ListItemButton";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";

interface FileExplorerProps {
    style: React.CSSProperties | undefined
}

const FileExplorer = (props: FileExplorerProps) => {
    return (
        <List
            style={ props.style } 
        >
            <ListItemButton
                sx={{
                    margin: 0,
                    padding: "5px 0 5px 0",
                    pl: 0,
                }}
            >
                <FolderOpenIcon
                    sx={{
                        padding: "0 5px 0 5px"
                    }} 
                />
                test
            </ListItemButton>
        </List>
    );
};

export default FileExplorer;
