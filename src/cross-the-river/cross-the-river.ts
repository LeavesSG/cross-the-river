import { BROTHER, FATHER, JIE_GE, MOTHER, Person, SISTER } from "./type";

interface Vert {
    thisSide: Person[];
    thatSide: Person[];
    direction: true;
}

type BoatDrive = [Person, Person | undefined];

type TupleStartWith<T extends R, U extends R[], R> = [T, ...U];

type PickPass<T extends Person[]> = T extends TupleStartWith<infer F, infer R, Person>
    ? _PickPass<R, [], T, F, []>
    : [];
type _PickPass<
    T extends Person[],
    UL extends Person[],
    UR extends Person[],
    D extends Person,
    Picked extends BoatDrive[]
> = UR extends TupleStartWith<infer N, infer R, Person>
    ? _PickPass<T, [...UL, N], R, D, [...Picked, [D, N extends D ? undefined : N]]>
    : T extends TupleStartWith<infer N, infer R, Person>
    ? _PickPass<R, [], UL, N, Picked>
    : Picked;

type START = {
    thisSide: [FATHER, MOTHER, BROTHER, SISTER, JIE_GE];
    thatSide: [];
    direction: true;
};

type TARGET = {
    thisSide: [];
    thatSide: [FATHER, MOTHER, BROTHER, SISTER, JIE_GE];
    direction: false;
};

type getAdj<T extends Vert> = {};

type s = [1, 2];
type z = [1, 2, 3];
