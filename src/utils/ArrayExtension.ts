export { }


type callback<T> = (data: T) => string | number;

export type MultiOrderProps = {
    properties: {
        property: string,
        type?: "ASC" | "DESC"
    }[]
}

declare global {
    interface Array<T> {
        toDict(cb: callback<T>): { [key: string]: T }
        toArrayDict(cb: callback<T>): { [key: string]: T[] }
        descompose<G>(): G[]
    }
}

Array.prototype.toDict = function <T>(this: Array<T>, cb: callback<T>) {
    const data: {[key: string]: T} = {};

    this.map(d => {
        const key = cb(d);
        if (data[key] === undefined) {
            data[key] = d;
        }
    });

    return data;
}

Array.prototype.toArrayDict = function <T>(this: Array<T>, cb: callback<T>) {
    var dict: { [key: string]: T[] } = {};
    this.forEach(item => {
        const _key = cb(item);
        if (_key) {
            if (dict[_key]) {
                dict[_key].push(item);
            } else {
                dict[_key] = [item];
            }
        }
    });
    return dict;
}

Array.prototype.descompose = function <T>(this: Array<Array<T>>) {
    const final: T[] = [];

    this.forEach(item => {
        if(Array.isArray(item)) {
            final.push(...item);
        } else {
            final.push(item);
        }
    });

    return final;
}
