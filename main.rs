fn main() {
    let mut running = true;
    let mut ip = 0;
    let mut sp = -1;

    let mut stack = [0; 256];

    enum InstructionSet {
        PSH,
        ADD,
        POP,
        HLT,
    }

    const PROGRAM: [InstructionSet; 7] = [
        InstructionSet::PSH,
        InstructionSet::PSH,
        InstructionSet::ADD,
        InstructionSet::POP,
        InstructionSet::HLT,
    ];

    fn fetch(program: &[InstructionSet], ip: &mut usize) -> &InstructionSet {
        let instruction = &program[*ip];
        *ip += 1;
        instruction
    }

    fn eval(instruction: &InstructionSet, running: &mut bool, ip: &mut usize, sp: &mut isize, stack: &mut [isize; 256]) {
        match instruction {
            InstructionSet::HLT => {
                *running = false;
                println!("done");
            }
            InstructionSet::PSH => {
                *sp += 1;
                stack[*sp as usize] = PROGRAM[*ip] as isize;
                *ip += 1;
            }
            InstructionSet::POP => {
                let val_popped = stack[*sp as usize];
                *sp -= 1;
                println!("popped {}", val_popped);
            }
            InstructionSet::ADD => {
                let a = stack[*sp as usize];
                *sp -= 1;
                let b = stack[*sp as usize];
                *sp -= 1;
                let result = b + a;
                *sp += 1;
                stack[*sp as usize] = result;
            }
        }
    }

    while running {
        let instruction = fetch(&PROGRAM, &mut ip);
        eval(instruction, &mut running, &mut ip, &mut sp, &mut stack);
        ip += 1;
    }
}
