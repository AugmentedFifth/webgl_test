export class Uint8Vec {
    private _data: Uint8Array;
    private _len:  number;

    public constructor(size: number) {
        this._data = new Uint8Array(size);
        this._len = 0;
    }

    public clear(): void {
        this._len = 0;
    }

    public get(index: number): number {
        return this._data[index];
    }

    public peek(): number {
        return this._data[this._len - 1];
    }

    public pop(): number {
        return this._data[--this._len];
    }

    public push(b: number): void {
        if (this._len === this._data.length) {
            const new_data = new Uint8Array(this._len * 2);
            for (let i = 0; i < this._len; ++i) {
                new_data[i] = this._data[i];
            }
            this._data = new_data;
        }

        this._data[this._len++] = b;
    }

    public get data(): Uint8Array {
        return this._data;
    }

    public get len(): number {
        return this._len;
    }
}

export class Uint16Vec {
    private _data: Uint16Array;
    private _len:  number;

    public constructor(size: number) {
        this._data = new Uint16Array(size);
        this._len = 0;
    }

    public clear(): void {
        this._len = 0;
    }

    public get(index: number): number {
        return this._data[index];
    }

    public peek(): number {
        return this._data[this._len - 1];
    }

    public pop(): number {
        return this._data[--this._len];
    }

    public push(b: number): void {
        if (this._len === this._data.length) {
            const new_data = new Uint16Array(this._len * 2);
            for (let i = 0; i < this._len; ++i) {
                new_data[i] = this._data[i];
            }
            this._data = new_data;
        }

        this._data[this._len++] = b;
    }

    public get data(): Uint16Array {
        return this._data;
    }

    public get len(): number {
        return this._len;
    }
}
