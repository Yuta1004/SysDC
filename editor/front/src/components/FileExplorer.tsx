import { useEffect, useState } from "react";

import List from "@mui/material/List";
import ListItemButton from "@mui/material/ListItemButton";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import TextSnippetIcon from "@mui/icons-material/TextSnippet";
import MyFileSystem, { MyNode, MyLeaf } from "../filesystem/MyFileSystem";

interface FileExplorerProps {
    style: React.CSSProperties | undefined,
    fs: MyFileSystem,
    onSelect: (path: string) => void
}

const FileExplorer = (props: FileExplorerProps) => {
    const [entries, setEntries] = useState<JSX.Element[]>([]);

    const createFsEntry = (node: MyNode, depth: number): JSX.Element[] => {
        const dirEntries = Array.from(node.nodes).map(([_, node]) => {
            return (<>
                <ListItemButton
                    sx={{
                        margin: 0,
                        padding: "5px 0 5px 0",
                        pl: depth*2,
                    }}
                    onClick={() => props.onSelect(node.name) }
                >
                    <FolderOpenIcon
                        sx={{
                            padding: "0 5px 0 5px"
                        }} 
                    />
                    { node.name.split("/").slice(-1)[0] }
                </ListItemButton>
                <List
                    style={props.style}
                >
                    {[ ...createFsEntry(node, depth+1) ]}
                </List>
            </>);
        });

        const fileEntries = Array.from(node.leaves).map(([_, node]) => {
            return (<>
                <ListItemButton
                    sx={{
                        margin: 0,
                        padding: "5px 0 5px 0",
                        pl: depth*2,
                    }}
                    onClick={() => props.onSelect(node.name) }
                >
                    <TextSnippetIcon
                        sx={{
                            padding: "0 5px 0 5px"
                        }} 
                    />
                    { node.name.split("/").slice(-1)[0] }
                </ListItemButton>
            </>);
        });

        return dirEntries.concat(fileEntries);
    };

    useEffect(() => {
        setEntries(createFsEntry(props.fs.root, 0));
    }, [props.fs]);

    return (
        <List
            style={ props.style } 
        >
            {[ ...entries ]}
        </List>
    );
};

export default FileExplorer;
