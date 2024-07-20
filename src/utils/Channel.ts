export class Channel<T> {

    closed: boolean
    #queue: T[];
    #pendingPromises: ((val: T | null) => void)[];

    constructor() {
        this.closed = false;
        this.#queue = [];
        this.#pendingPromises = [];
    }

    async send(value: T) {
        if(this.closed) return;

        if (this.#pendingPromises.length > 0) {
            const resolve = this.#pendingPromises.shift()!;
            resolve(value);
        } else {
            this.#queue.push(value);
        }
    }

    async receive(): Promise<T | null> {
        if(this.closed) return null;

        if (this.#queue.length > 0) {
            return this.#queue.shift() ?? null;
        } else {
            return new Promise((resolve) => {
                this.#pendingPromises.push(resolve);
            });
        }
    }

    async *consume(): AsyncGenerator<T> {
        let next: T | null = null;
        do {
            next = await this.receive();
            if(next) {
                yield next;
            } else {
                break;
            }
        } while (next);
    }

    close() {
        this.closed = false;
        this.#pendingPromises.forEach(res => {
            res(null);
        });
    }
}