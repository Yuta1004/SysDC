/* structure definitions */

export type Name = string;

export type Type = string;

export type SysDCSystem = {
    readonly units: SysDCUnit[]
}

export type SysDCUnit = {
    readonly name: Name,
    readonly data: SysDCData[],
    readonly modules: SysDCModule[]
}

export type SysDCData = {
    readonly name: Name,
    readonly members: [Name, Type][]
}

export type SysDCModule = {
    readonly name: Name,
    readonly functions: SysDCFunction[]
}

export type SysDCFunction = {
    readonly name: Name,
    readonly args: [Name, Type][],
    readonly return: [Name, Type],
    readonly annotations: SysDCAnnotation[]
}

export type SysDCAnnotation =
    SysDCAnnotationAffect |
    SysDCAnnotationModify |
    SysDCAnnotationSpawn

export type SysDCAnnotationAffect = {
    readonly func: [Name, Type],
    readonly args: [Name, Type][]
}

export type SysDCAnnotationModify = {
    readonly target: [Name, Type],
    readonly uses: [Name, Type][]
}

export type SysDCAnnotationSpawn = {
    readonly result: [Name, Type],
    readonly details: SysDCSpawnDetail[]
};

export type SysDCSpawnDetail =
    SysDCSpawnDetailUse |
    SysDCSpawnDetailReturn |
    SysDCSpawnDetailLetTo

export type SysDCSpawnDetailUse = readonly [Name, Type]

export type SysDCSpawnDetailReturn = readonly [Name, Type]

export type SysDCSpawnDetailLetTo = {
    readonly name: Name,
    readonly func: [Name, Type],
    readonly args: [Name, Type][]
}

/* converters */

export function convert(obj: object): SysDCSystem {
    return {
        units: obj["units"].map(__convert_unit)
    };
}

function __convert_unit(obj: object): SysDCUnit {
    return {
        name: __convert_name(obj["name"]),
        data: obj["data"].map(__convert_data),
        modules: obj["modules"].map(__convert_module)
    };
}

function __convert_data(obj: object): SysDCData {
    return {
        name: __convert_name(obj["name"]),
        members: obj["members"].map(__convert_name_type)
    };
}

function __convert_module(obj: object): SysDCModule {
    return {
        name: __convert_name(obj["name"]),
        functions: obj["functions"].map(__convert_function)
    };
}

function __convert_function(obj: object): SysDCFunction  {
    return {
        name: __convert_name(obj["name"]),
        args: obj["args"].map(__convert_name_type),
        return: __convert_name_type(obj["returns"]),
        annotations: obj["annotations"].map(__convert_annotation)
    };
}

function __convert_annotation(obj: object): SysDCAnnotation {
    if (obj["Affect"] != undefined) {
        return {
            func: __convert_name_type(obj["Affect"]["func"]),
            args: obj["Affect"]["args"].map(__convert_name_type)
        };
    }
    if (obj["Modify"] != undefined) {
        return {
            target: __convert_name_type(obj["Modify"]["target"]),
            uses: obj["Modify"]["uses"].map(__convert_name_type)
        };
    }
    if (obj["Spawn"] != undefined) {
        return {
            result: __convert_name_type(obj["Spawn"]["result"]),
            details: obj["Spawn"]["details"].map(__convert_spawn_detail)
        };
    }
}

function __convert_spawn_detail(obj: Object): SysDCSpawnDetail {
    if (obj["Use"] != undefined) {
        return __convert_name_type(obj["Use"]);
    }
    if (obj["Return"] != undefined) {
        return __convert_name_type(obj["Return"]);
    }
    if (obj["LetTo"] != undefined) {
        return {
            name: __convert_name(obj["LetTo"]["name"]),
            func: __convert_name_type(obj["LetTo"]["func"]),
            args: obj["LetTo"]["args"].map(__convert_name_type)
        };
    }
}

function __convert_name(obj: object): Name {
    return obj["namespace"] + "." + obj["name"];
}

function __convert_type(obj: object): Type {
    if (obj["refs"] == null) {
        return obj["kind"];
    }
    return __convert_name(obj["refs"]);
}

function __convert_name_type(obj: object): [Name, Type] {
    return [__convert_name(obj[0]), __convert_type(obj[1])];
}
