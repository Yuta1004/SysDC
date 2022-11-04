export type MyNode = {
    name: string,
    nodes: Map<string, MyNode>,
    leaves: Map<string, MyLeaf>
}

export type MyLeaf = {
    name: string,
    body: string
}

export class MyFileSystem {
    root: MyNode

    constructor() {
        this.root = {
            name: "/",
            nodes: new Map(),
            leaves: new Map()
        };
    }

    mkdir(path: string): boolean {
        const [result, foundNode] = this.getNode(this.root, path.split("/").slice(1).join("/"));
        if (result) {
            foundNode.nodes.set(path.split("/").slice(-1)[0], {
                name: path,
                nodes: new Map(),
                leaves: new Map()
            });
        }
        return result;
    }

    mkfile(path: string, body: string): boolean {
        const dirPath = path.split("/").slice(1, -1).join("/");
        const filename = path.split("/").slice(-1)[0];
        const [result, foundNode] = this.getNode(this.root, dirPath);
        if (result) {
            foundNode.leaves.set(filename, {
                name: dirPath + "/" + filename,
                body: body
            });
        }
        return result;
    }

    read(path: string): string|undefined {
        const dirPath = path.split("/").slice(1, -1).join("/");
        const filename = path.split("/").slice(-1)[0];
        const [result, foundNode] = this.getNode(this.root, dirPath);
        if (result) {
            const leaf = foundNode.leaves.get(filename);
            return leaf ? leaf.body : undefined;
        }
        return undefined;
    }

    getNode(parNode: MyNode, path: string): [boolean, MyNode] {
        const splittedPath = path.split("/");
        const dir = parNode.nodes.get(splittedPath[0]);
        if (dir !== undefined) {
            if (splittedPath.length === 1) {
                return [true, dir];
            } else {
                return this.getNode(dir, splittedPath.slice(1).join("/"));
            }
        }
        return [splittedPath.length === 1, parNode];
    }
}


export default MyFileSystem;
