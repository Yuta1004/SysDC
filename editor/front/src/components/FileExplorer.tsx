import { useEffect, useState, useContext } from "react";

import Box from "@mui/material/Box";
import List from "@mui/material/List";
import ListItemButton from "@mui/material/ListItemButton";
import Stack from "@mui/material/Stack";
import Button from "@mui/material/Button";
import Divider from "@mui/material/Divider";
import FolderOpenIcon from "@mui/icons-material/FolderOpen";
import TextSnippetIcon from "@mui/icons-material/TextSnippet";
import CreateNewFolderOutlinedIcon from '@mui/icons-material/CreateNewFolderOutlined';
import NoteAddOutlinedIcon from '@mui/icons-material/NoteAddOutlined';

import { MyNode } from "../filesystem/MyFileSystem";
import { FSContext, TargetFileContext } from "../App";

interface FileExplorerProps {
    width: string
}

const FileExplorer = (props: FileExplorerProps) => {
    const fs = useContext(FSContext);
    const [_targetFile, setTargetFile] = useContext(TargetFileContext);

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
                    onClick={() => setTargetFile(node.name) }
                >
                    <FolderOpenIcon
                        sx={{
                            padding: "0 5px 0 5px"
                        }} 
                    />
                    { node.name.split("/").slice(-1)[0] }
                </ListItemButton>
                <List>
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
                    onClick={() => setTargetFile(node.name) }
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

    const createDirectory = () => {
        const path = prompt("新規作成するディレクトリのパスを入力してください");
        if (path !== null && path !== "") {
            fs.mkdir(path);
        }
        setEntries(createFsEntry(fs.root, 0));
    };

    const createFile = () => {
        const path = prompt("新規作成するファイルのパスを入力してください");
        if (path !== null && path !== "") {
            fs.mkfile(path, "");
        }
        setEntries(createFsEntry(fs.root, 0));
    };

    useEffect(() => {
        setEntries(createFsEntry(fs.root, 0));
    }, [fs]);

    return (
        <Box
            style={{
                width: props.width
            }} 
        >
            <Stack
                direction="row"
                justifyContent="center"
                spacing={2}
                sx={{
                    padding: "5px"
                }}
            >
                <Button
                    variant="outlined"
                    size="small"
                    onClick={ createDirectory }
                >
                    <CreateNewFolderOutlinedIcon/>
                </Button>
                <Button
                    variant="outlined"
                    size="small"
                    onClick={ createFile }
                >
                    <NoteAddOutlinedIcon/>
                </Button>
            </Stack>
            <Divider/>
            <List>
                {[ ...entries ]}
            </List>
        </Box>
    );
};

export default FileExplorer;
