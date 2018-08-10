export class Event {
    private readonly _opcode: number;
    private readonly _payload: Uint8Array;

    public constructor(opcode: number, payload: Uint8Array) {
        this._opcode = opcode;
        this._payload = payload;
    }

    public get opcode(): number {
        return this._opcode;
    }

    public get payload(): Uint8Array {
        return this._payload;
    }
}

export class EventQueue {
    private readonly _data: Event[];

    public constructor() {
        this._data = [];
    }

    public clear(): void {
        this._data.length = 0;
    }

    public get(index: number): Event {
        return this._data[index];
    }

    public push(event: Event): void {
        this._data.push(event);
    }

    public get len(): number {
        return this._data.length;
    }
}

export const KEY_DOWN = 0x01;
export const KEY_UP = 0x02;
export const MOUSE_MOVE = 0x03;

const key_code_map = new Map([
    ["KeyW", 0x01],
    ["KeyA", 0x02],
    ["KeyS", 0x03],
    ["KeyD", 0x04],
]);
export function get_key_code(code_str: string): number | undefined {
    return key_code_map.get(code_str);
}
