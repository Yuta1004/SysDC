import {
    Name, Type,
    SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCAnnotation, SysDCSpawnDetail
} from "./structure";

export default convert;

export function convert(obj: object): SysDCSystem {
    return {
        units: obj["units"].map(convertUnit)
    };
}

function convertUnit(obj: object): SysDCUnit {
    return {
        name: convertName(obj["name"]),
        data: obj["data"].map(convertData),
        modules: obj["modules"].map(convertModule)
    };
}

function convertData(obj: object): SysDCData {
    return {
        name: convertName(obj["name"]),
        members: obj["members"].map(convertNameType)
    };
}

function convertModule(obj: object): SysDCModule {
    return {
        name: convertName(obj["name"]),
        functions: obj["functions"].map(convertFunction)
    };
}

function convertFunction(obj: object): SysDCFunction  {
    return {
        name: convertName(obj["name"]),
        args: obj["args"].map(convertNameType),
        return: convertNameType(obj["returns"]),
        annotations: obj["annotations"].map(convertAnnotation)
    };
}

function convertAnnotation(obj: object): SysDCAnnotation {
    if (obj["Affect"] != undefined) {
        return {
            func: convertNameType(obj["Affect"]["func"]),
            args: obj["Affect"]["args"].map(convertNameType)
        };
    }
    if (obj["Modify"] != undefined) {
        return {
            target: convertNameType(obj["Modify"]["target"]),
            uses: obj["Modify"]["uses"].map(convertNameType)
        };
    }
    if (obj["Spawn"] != undefined) {
        return {
            result: convertNameType(obj["Spawn"]["result"]),
            details: obj["Spawn"]["details"].map(convertSpawnDetail)
        };
    }
}

function convertSpawnDetail(obj: Object): SysDCSpawnDetail {
    if (obj["Use"] != undefined) {
        return convertNameType(obj["Use"]);
    }
    if (obj["Return"] != undefined) {
        return convertNameType(obj["Return"]);
    }
    if (obj["LetTo"] != undefined) {
        return {
            name: convertName(obj["LetTo"]["name"]),
            func: convertNameType(obj["LetTo"]["func"]),
            args: obj["LetTo"]["args"].map(convertNameType)
        };
    }
}

function convertName(obj: object): Name {
    return obj["namespace"] + "." + obj["name"];
}

function convertType(obj: object): Type {
    if (obj["refs"] == null) {
        return obj["kind"];
    }
    return convertName(obj["refs"]);
}

function convertNameType(obj: object): [Name, Type] {
    return [convertName(obj[0]), convertType(obj[1])];
}
