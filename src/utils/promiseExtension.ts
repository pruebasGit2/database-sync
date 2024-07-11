import { Delay } from "./delay";

export { }

declare global {
    interface Promise<T> {
        wait(ms: number): Promise<T>
    }
}

Promise.prototype.wait = function <T>(this: Promise<T>, ms: number) {
    return new Promise(async(res, rej) => {
        await Delay(ms);
        this.then(res);
        this.catch(rej);
    });
}