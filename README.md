# RSIM
A RISC-V simulator, which can simulate the Base RV32I ISA. The simulator takes a ```.bin``` file as input, created from the [RASM](https://github.com/JagratPatkar/RASM) assembler.

## Supported Instructions

__R Type__: ``ADD``, ```SUB```, ```SIL```, ```SLT```, ```SLTU```, ```XOR```, ```SRL```, ```SRA```, ```OR```, ```AND```

__I Type__: ``ADDI``, ```JALR```, ```SLLI```, ```SLTI```, ```SLTIU```, ```XORI```, ```SRLI```, ```SRAI```, ```ORI```, ```ANDI```, ```LW```, ```LH```, ```LB```

__S Type__: ``SB``, ```SH```, ```SW```

__B Type__: ``BEQ``, ```BNE```, ```BLT```, ```BGE```, ```BLTU```, ```BGEU```

__J Type__: ``JAL``

__U Type__: ``LUI``, ```AUIPC```