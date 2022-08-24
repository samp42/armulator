export class RegisterArray {
    private registers: Map<string, string>;

    constructor(
        initialR0: string,
        initialR1: string,
        initialR2: string,
        initialR3: string,
        initialR4: string,
        initialR5: string,
        initialR6: string,
        initialR7: string,
        initialR8: string,
        initialR9: string,
        initialR10: string,
        initialR11: string,
        initialR12: string,
        initialSP: string,
        initialLR: string,
        initialPC: string,
        initialCPSR: string,
        initialSPSR: string,
    ) {
        this.registers = new Map<string, string>();

        this.registers.set('R0', initialR0);
        this.registers.set('R1', initialR1);
        this.registers.set('R2', initialR2);
        this.registers.set('R3', initialR3);
        this.registers.set('R4', initialR4);
        this.registers.set('R5', initialR5);
        this.registers.set('R6', initialR6);
        this.registers.set('R7', initialR7);
        this.registers.set('R8', initialR8);
        this.registers.set('R9', initialR9);
        this.registers.set('R10', initialR10);
        this.registers.set('R11', initialR11);
        this.registers.set('R12', initialR12);
        this.registers.set('SP', initialSP);
        this.registers.set('LR', initialLR);
        this.registers.set('PC', initialPC);
        this.registers.set('CPSR', initialCPSR);
        this.registers.set('SPSR', initialSPSR);
    }

    setRegister(register: string, value: string) {
        if (this.registers.has(register)) {
            this.registers.set(register, value);
        }
    }

    // getRegister(register: string): string {
    //     return this.registers.get(register);
    // }
}
