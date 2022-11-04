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
            <ListItemButton sx={{ pl: 0 }}>
                <FolderOpenIcon/>
                test
            </ListItemButton>
        </List>
    );
};

export default FileExplorer;
