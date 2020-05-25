use strip_shared::compiler::compile;
use strip_shared::parser::parse;
use strip_shared::vm::*;

#[test]
fn test_directives() {
  assert_vm_state(
    &"
    .byte 0xff
    .zero 1
    .half 0xfefe
    .word 0xfafbfcfd
    .equ FOO 42

    li s0 FOO
  ",
    1,
    [0, 0, 42, 0, 0, 0, 0, 0],
    vec![255, 0, 254, 254, 250, 251, 252, 253],
  )
}

#[test]
fn test_string_directive() {
  assert_vm_state(
    &"
    .zero 3
    message: 
      .string \"Hello\"

    lb s0 message
    lb s1 2(message)
    li s2 4
    lb s2 message(s2)
  ",
    4,
    [0, 0, 72, 108, 111, 0, 0, 0],
    vec![0, 0, 0, 72, 101, 108, 108, 111],
  );
}

#[test]
fn test_noop() {
  assert_vm_state(
    &"
    nop
    nop
    nop
    nop
  ",
    4,
    [0; 8],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_ecall() {
  assert_vm_state(
    &"
    .equ ECALL_RAND 0xff

    ecall zero ECALL_RAND
    ecall s0 ECALL_RAND
    ecall s1 ECALL_RAND(zero)
    li s5 1
    ecall s3 ECALL_RAND(s5)
    ecall s2 ECALL_RAND(s5)
  ",
    6,
    [0, 0, 48271, 48271, 96542, 96542, 0, 1],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_loads() {
  assert_vm_state(
    &"
    li s0 0x5678
    li s0 0x5678
    label: 
      lui s0 0x1234
      la s1 label
      la s2 (pc)
      la s3 -1(pc)
  ",
    6,
    [0, 0, 0x1234_5678, 2, 4, 4, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_mem() {
  assert_vm_state(
    &"
    .equ MAGIC 0x2
    li s0 0xaf
    sb s0 MAGIC
    sb s0 1(MAGIC)
    lbu s1 MAGIC
    lb s2 MAGIC
    lh s3 MAGIC
    lhu s4 MAGIC
    lw s5 -2(MAGIC)
    sw s5 2(MAGIC)
  ",
    9,
    [0, 0, 0xaf, 0xaf, -81, -20561, 0xafaf, 0xafaf],
    vec![0, 0, 0xaf, 0xaf, 0, 0, 0xaf, 0xaf],
  );
}

#[test]
fn test_add() {
  assert_vm_state(
    &"
    addi s0 s0 1
    add s1 s0 s0
    inc s2
    dec s3
    dec s3
  ",
    5,
    [0, 0, 1, 2, 1, -2, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_and() {
  assert_vm_state(
    &"
    li s0 0b1111
    li s1 0b10
    and s0 s0 s1
    andi s1 s1 0b11
  ",
    4,
    [0, 0, 2, 2, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_mul() {
  assert_vm_state(
    &"
    li s0 100
    li s1 500
    li s2 -2
    mul s0 s0 s1
    mul s1 s1 s2
  ",
    5,
    [0, 0, 50000, -1000, -2, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_or() {
  assert_vm_state(
    &"
    li s0 0b101
    li s1 0b010
    or s0 s0 s1
    ori s1 s1 0b11
  ",
    4,
    [0, 0, 7, 3, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sub() {
  assert_vm_state(
    &"
    li s0 42
    li s1 40
    sub s1 s0 s1
    sub s2 s1 s0
  ",
    4,
    [0, 0, 42, 2, -40, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_xor() {
  assert_vm_state(
    &"
    li s0 0b101
    inc s1
    xor s0 s0 s1
    xori s1 s1 0b1111
  ",
    4,
    [0, 0, 4, 14, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sll() {
  assert_vm_state(
    &"
    li s0 1
    li s1 3
    sll s0 s0 s1
    slli s2 s0 8
    li s3 -1
    slli s4 s3 8
  ",
    6,
    [0, 0, 8, 3, 2048, -1, -256, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_srl() {
  assert_vm_state(
    &"
    li s0 0b100000000
    li s1 3
    srl s2 s0 s1
    srli s3 s0 8
    li s4 -32
    srli s5 s4 3
  ",
    6,
    [0, 0, 256, 3, 32, 1, -32, -4],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sra() {
  assert_vm_state(
    &"
    li s0 0b100000000
    li s1 3
    sra s2 s0 s1
    li s3 -32
    sra s4 s3 s1
  ",
    5,
    [0, 0, 256, 3, 32, -32, -4, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_slt() {
  assert_vm_state(
    &"
    li s0 1
    li s1 -4
    slt s1 s1 s0
    slt s2 s0 s1
  ",
    4,
    [0, 0, 1, 1, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sltiu() {
  assert_vm_state(
    &"
    li s0 1
    sltiu s1 s0 -2
    sltiu s2 s0 2
  ",
    3,
    [0, 0, 1, 1, 1, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sltu() {
  assert_vm_state(
    &"
    li s0 -4
    sltu s1 s0 s1
    sltu s2 s1 s0
  ",
    3,
    [0, 0, -4, 0, 1, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_seqz() {
  assert_vm_state(
    &"
    seqz s0 s0
    seqz s0 s0
    seqz s1 s1
  ",
    3,
    [0, 0, 0, 1, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_snez() {
  assert_vm_state(
    &"
    li s0 42
    snez s0 s0
    snez s1 s1
  ",
    3,
    [0, 0, 1, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sltz() {
  assert_vm_state(
    &"
    li s0 -2
    sltz s0 s0
    sltz s1 s1
  ",
    3,
    [0, 0, 1, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_sgtz() {
  assert_vm_state(
    &"
    li s0 2
    sgtz s0 s0
    sgtz s1 s1
  ",
    3,
    [0, 0, 1, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_beq() {
  assert_vm_state(
    &"
    beq s0 s0 2(pc)
    li s1 42
  ",
    2,
    [0, 1, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_bne() {
  assert_vm_state(
    &"
    bne s0 s0 2(pc)
    li s1 42
  ",
    2,
    [0, 0, 0, 42, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_bge() {
  assert_vm_state(
    &"
    bge s0 s0 2(pc)
    li s1 42
    bge s1 s0 2(pc)
    li s1 42
  ",
    4,
    [0, 3, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_blt() {
  assert_vm_state(
    &"
    blt s0 s0 2(pc)
    li s1 42
    blt s1 s0 2(pc)
    li s2 43
  ",
    4,
    [0, 0, 0, 42, 43, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_bgeu() {
  assert_vm_state(
    &"
    bgeu s0 s0 load
    nop
    li s5 2
    nop

    load: 
      li s1 -42

    bgeu s1 s0 2(pc)
    li s2 43
  ",
    7,
    [0, 6, 0, -42, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

#[test]
fn test_bltu() {
  assert_vm_state(
    &"
    bltu s0 s0 load
    nop
    li s5 2
    nop

    load: 
      li s1 -42

    bltu s0 s1 2(pc)
    li s2 43
  ",
    7,
    [0, 6, 0, -42, 0, 0, 0, 2],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  );
}

fn assert_vm_state(code: &str, target_pc: usize, target_regs: [i32; 8], target_mem: Vec<u8>) {
  let (pc, reg, ram) = spin_vm(1, code).unwrap();
  assert_eq!(pc, target_pc);
  assert_eq!(reg, target_regs);
  assert_eq!(ram, target_mem);
}

fn spin_vm(spins: u16, code: &str) -> Result<(usize, [i32; 8], Vec<u8>), VMError> {
  let exprs = parse(&code).unwrap();
  let bytecode = compile(&exprs).unwrap();
  let mut vm = VM::new(TestEnv::new(8));
  vm.load(&bytecode)?;
  let mut spins = spins;
  while spins > 0 {
    vm.respin()?;
    spins -= 1;
  }
  let pc = *vm.get_pc();
  let reg = *vm.get_reg();
  let ram = vm.get_env().ram.clone();
  Ok((pc, reg, ram))
}

struct TestEnv {
  ram: Vec<u8>,
}

impl TestEnv {
  fn new(ram_size: u16) -> Self {
    TestEnv {
      ram: vec![0; ram_size as usize],
    }
  }
}

impl core::fmt::Debug for TestEnv {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self.ram)
  }
}

impl Env for TestEnv {
  type Error = ();

  fn reset(&mut self) {
    self.ram = vec![0; self.ram.len()];
  }

  fn mem_fetch(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    let end = offset + buf.len();
    if end > self.ram.len() {
      return Err(());
    }
    buf.copy_from_slice(&self.ram[offset..end]);
    Ok(())
  }

  fn mem_set(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    let end = offset + val.len();
    if end > self.ram.len() {
      return Err(());
    }
    self.ram[offset..end].copy_from_slice(val);
    Ok(())
  }

  fn ecall(&mut self, ecall: i32, param: i32) -> Result<i32, Self::Error> {
    if ecall == 0xff {
      let (rand, _) = (param + 1).overflowing_mul(48271);
      return Ok(rand % 0x7fff_ffff);
    }
    Ok(0)
  }
}
