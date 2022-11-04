const SysDCSyntaxHighlight = {
    "start": [
        {
            token: "comment",
            regex: "%",
        },
        {
            token: "keyword.control",
            regex: "unit|from|import"
        },
        {
            token: "keyword.other",
            regex: "use|let|return"
        },
        {
            token: "storage",
            regex: "data",
            next: "dataName",
        },
        {
            token: "storage",
            regex: "module",
            next: "moduleName"
        },
        {
            token: "storage",
            regex: "proc|func",
            next: "funcName"
        },
        {
            token: "meta.name.tag",
            regex: "@[a-zA-Z_]*",
        }
    ],
    "comment": [
        {
            token: "comment",
            regex: "%",
            next: "start"
        },
        {
            defaultToken: "comment"
        }
    ],
    "dataName": [
        {
            token: "entity.name.function",
            regex: "[a-zA-Z_][a-zA-Z0-9]*",
        },
        {
            token: "entity",
            regex: "{",
            next: "dataBody1",
        },
    ],
    "dataBody1": [
        {
            token: "entity.name.tag",
            regex: "[a-zA-Z_][a-zA-Z0-9]*",
        },
        {
            token: "entity",
            regex: ":",
            next: "dataBody2",
        },
        {
            token: "entity",
            regex: "}",
            next: "start"
        },
        {
            defaultToken: "dataBody1"
        }
    ],
    "dataBody2": [
        {
            token: "entity.name.type",
            regex: "[a-zA-Z_][a-zA-Z0-9]*",
        },
        {
            token: "entity",
            regex: ",",
            next: "dataBody1"
        },
        {
            token: "entity",
            regex: "}",
            next: "start"
        },
        {
            defaultToken: "dataBody2"
        }
    ],
    "moduleName": [
        {
            token: "entity.name.function",
            regex: "[a-zA-Z_][a-zA-Z0-9_]*",
        },
        {
            token: "entity",
            regex: "{",
            next: "start"
        },
        {
            defaultToken: "moduleName"
        }
    ],
    "funcName": [
        {
            token: "entity.name.function",
            regex: "[a-zA-Z_][a-zA-Z0-9_]*",
        },
        {
            token: "entity",
            regex: "\\(",
            next: "funcArgs1"
        },
        {
            defaultToken: "funcName"
        }
    ],
    "funcArgs1": [
        {
            token: "entity.name.tag",
            regex: "[a-zA-Z_][a-zA-Z0-9]*",
        },
        {
            token: "entity",
            regex: ":",
            next: "funcArgs2",
        },
        {
            token: "entity",
            regex: "\\)",
            next: "funcRet"
        },
        {
            defaultToken: "funcArgs1"
        }
    ],
    "funcArgs2": [
        {
            token: "entity.name.type",
            regex: "[a-zA-Z_][a-zA-Z0-9]*",
        },
        {
            token: "entity",
            regex: ",",
            next: "funcArgs1"
        },
        {
            token: "entity",
            regex: "\\)",
            next: "funcRet"
        },
        {
            defaultToken: "funcArgs2"
        }
    ],
    "funcRet": [
        {
            token: "entity.name.type",
            regex: "[a-zA-Z_][a-zA-Z0-9]*",
        },
        {
            token: "entity",
            regex: "\\{",
            next: "start",
        },
        {
            defaultToken: "funcRet"
        }
    ]
};

export default SysDCSyntaxHighlight;
