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
        const _mkdir = (dirPath: string, dirName: string) => {
            const [result, foundNode] = this.getNode(this.root, dirPath);
            if (result && !foundNode.nodes.has(dirName)) {
                foundNode.nodes.set(dirName, {
                    name: this.checkPath(dirPath+dirName),
                    nodes: new Map(),
                    leaves: new Map()
                });
            }
        };

        const _path = this.checkPath(path);
        var dirPath = "";
        _path.split("/").slice(1).forEach((elem) => {
            _mkdir(dirPath, elem+"");
            dirPath += elem + "/";
        });
        return true;
    }

    mkfile(path: string, body: string): boolean {
        const _path = this.checkPath(path);

        const dirPath = _path.split("/").slice(1, -1).join("/");
        if (dirPath !== "") {
            this.mkdir(dirPath);
        }

        const filename = _path.split("/").slice(-1)[0];
        const [result, foundNode] = this.getNode(this.root, dirPath);
        if (result) {
            foundNode.leaves.set(filename, {
                name: _path,
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

    readAll(): MyLeaf[] {
        const recuresiveSearch = (node: MyNode, foundLeaves: MyLeaf[]): MyLeaf[] => {
            const leaves = Array.from(node.nodes)
                                .map(([_, node]) => recuresiveSearch(node, foundLeaves))
                                .flat();
            const leaves2 = Array.from(node.leaves.values());
            return foundLeaves.concat(leaves.concat(leaves2));
        };
        return recuresiveSearch(this.root, []);
    }

    private checkPath(path: string): string {
        if (path.at(0) !== "/") {
            return "/" + path;
        } else {
            return path;
        }
    }

    private getNode(parNode: MyNode, path: string): [boolean, MyNode] {
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
