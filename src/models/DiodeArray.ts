export class DiodeArray {
    diodes: boolean[];

    constructor(diodes: boolean[]) {
        if (diodes.length !== 8) throw Error("invalid length for LEDs");

        this.diodes = diodes;
    }

    setDiode(index: number, state: boolean) {
        this.diodes[index] = state;
    }
    
}