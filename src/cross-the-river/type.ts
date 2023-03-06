export interface Person {
    id: number;
}

enum Gender {
    Male,
    Female,
}

export interface Male extends Person {
    gender: Gender.Male;
}

export interface Female extends Person {
    gender: Gender.Female;
}

export type FATHER = {
    id: 0;
    gender: Gender.Male;
};

export type BROTHER = {
    id: 1;
    gender: Gender.Male;
};
export type JIE_GE = {
    id: 2;
    gender: Gender.Male;
};

export type MOTHER = {
    id: 3;
    gender: Gender.Male;
};

export type SISTER = {
    id: 4;
    gender: Gender.Male;
};
