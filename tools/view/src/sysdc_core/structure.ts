export default SysDCSystem;

export type Name = {
    readonly name: string,
    readonly namespace: string
}

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
