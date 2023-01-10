import {
    Name, Type,
    SysDCSystem, SysDCUnit, SysDCData, SysDCModule, SysDCFunction, SysDCAnnotation, SysDCSpawnDetail
} from "./structure";

export default convert;

export const convert = (obj: any): SysDCSystem => {
    return {
        units: obj["units"].map(convertUnit)
    };
}

const convertUnit = (obj: any): SysDCUnit => {
    return {
        name: convertName(obj["name"]),
        data: obj["data"].map(convertData),
        modules: obj["modules"].map(convertModule)
    };
}

const convertData = (obj: any): SysDCData => {
    return {
        name: convertName(obj["name"]),
        members: obj["members"].map(convertNameType)
    };
}

const convertModule = (obj: any): SysDCModule => {
    return {
        name: convertName(obj["name"]),
        functions: obj["functions"].map(convertFunction)
    };
}

const convertFunction = (obj: any): SysDCFunction => {
    return {
        name: convertName(obj["name"]),
        args: obj["args"].map(convertNameType),
        return: convertNameType(obj["returns"]),
        annotations: obj["annotations"].map(convertAnnotation)
    };
}

const convertAnnotation = (obj: any): SysDCAnnotation | undefined => {
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

const convertSpawnDetail = (obj: any): SysDCSpawnDetail | undefined => {
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

const convertName = (obj: any): Name => {
    return {
        fname: obj["namespace"]+"."+obj["name"],
        name: obj["name"],
        namespace: obj["namespace"]
    };
}

const convertType = (obj: any): Type => {
    if (obj["refs"] == null) {
        return obj["kind"];
    }
    return obj["refs"]["namespace"] + "." + obj["refs"]["name"];
}

const convertNameType = (obj: any): [Name, Type] => {
    return [convertName(obj[0]), convertType(obj[1])];
}
