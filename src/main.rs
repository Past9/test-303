#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f303_api::gpio::{
  gpioa::{Gpioa, Pa8AltFunc, Pa8Mco},
  gpioe::{
    Gpioe, Pe10Output, Pe11Output, Pe12Output, Pe13Output, Pe14Output, Pe15Output, Pe8Output,
    Pe9Output,
  },
  DigitalValue, OutputSpeed, OutputType, PullDirection,
};
use stm32f303_api::{Result, System};

#[entry]
#[no_mangle]
fn main() -> ! {
  if let Err(e) = run() {
    panic!(e.message);
  }

  loop {}
}

fn run() -> Result<()> {
  let mut program = Program::initialize()?;

  while program.should_continue {
    program.step()?;
  }

  program.shutdown()?;

  Ok(())
}

pub struct Program {
  should_continue: bool,
  count: u32,
  system: System,
  gpioa: Gpioa,
  mco_controller: McoController,
  gpioe: Gpioe,
  blinker: Blinker,
}
impl Program {
  pub fn initialize() -> Result<Self> {
    let mut system = System::new();

    let mut gpioa = system.activate_gpioa()?;
    let mco_controller = McoController::new(&mut gpioa)?;

    let mut gpioe = system.activate_gpioe()?;
    let blinker = Blinker::new(&mut gpioe)?;

    Ok(Self {
      should_continue: true,
      count: 0,
      system,
      gpioa,
      mco_controller,
      gpioe,
      blinker,
    })
  }

  pub fn step(&mut self) -> Result<()> {
    if self.count >= 9 {
      self.should_continue = false;
      return Ok(());
    }

    self.blinker.switch();
    for _ in 0..1000 {}

    self.count += 1;

    Ok(())
  }

  pub fn shutdown(mut self) -> Result<()> {
    self.blinker.return_hardware(&mut self.gpioe)?;
    self.system.deactivate_gpioe(self.gpioe)?;

    self.mco_controller.return_hardware(&mut self.gpioa)?;
    self.system.deactivate_gpioa(self.gpioa)?;

    Ok(())
  }
}

pub struct McoController {
  pa8: Pa8AltFunc<Pa8Mco>,
}
impl McoController {
  pub fn new(gpioa: &mut Gpioa) -> Result<Self> {
    let pa8 = gpioa
      .take_pa8()?
      .as_alt_func::<Pa8Mco>(PullDirection::Floating);
    Ok(Self { pa8 })
  }

  pub fn return_hardware(self, gpioa: &mut Gpioa) -> Result<()> {
    gpioa.return_pa8(self.pa8.teardown())?;

    Ok(())
  }
}

pub struct Blinker {
  which: WhichLed,
  pe8: Pe8Output,
  pe9: Pe9Output,
  pe10: Pe10Output,
  pe11: Pe11Output,
  pe12: Pe12Output,
  pe13: Pe13Output,
  pe14: Pe14Output,
  pe15: Pe15Output,
}
impl Blinker {
  pub fn new(gpioe: &mut Gpioe) -> Result<Self> {
    Ok(Self {
      which: WhichLed::Led3,
      pe8: gpioe.take_pe8()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe9: gpioe.take_pe9()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe10: gpioe.take_pe10()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe11: gpioe.take_pe11()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe12: gpioe.take_pe12()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe13: gpioe.take_pe13()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe14: gpioe.take_pe14()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
      pe15: gpioe.take_pe15()?.as_output(
        PullDirection::Floating,
        OutputType::PushPull,
        OutputSpeed::Low,
      ),
    })
  }

  pub fn switch(&mut self) {
    self
      .pe8
      .write(DigitalValue::from_bool(self.which == WhichLed::Led4));
    self
      .pe9
      .write(DigitalValue::from_bool(self.which == WhichLed::Led3));
    self
      .pe10
      .write(DigitalValue::from_bool(self.which == WhichLed::Led5));
    self
      .pe11
      .write(DigitalValue::from_bool(self.which == WhichLed::Led7));
    self
      .pe12
      .write(DigitalValue::from_bool(self.which == WhichLed::Led9));
    self
      .pe13
      .write(DigitalValue::from_bool(self.which == WhichLed::Led10));
    self
      .pe14
      .write(DigitalValue::from_bool(self.which == WhichLed::Led8));
    self
      .pe15
      .write(DigitalValue::from_bool(self.which == WhichLed::Led6));
    self.which = self.which.next();
  }

  pub fn return_hardware(self, gpioe: &mut Gpioe) -> Result<()> {
    gpioe.return_pe8(self.pe8.teardown())?;
    gpioe.return_pe9(self.pe9.teardown())?;
    gpioe.return_pe10(self.pe10.teardown())?;
    gpioe.return_pe11(self.pe11.teardown())?;
    gpioe.return_pe12(self.pe12.teardown())?;
    gpioe.return_pe13(self.pe13.teardown())?;
    gpioe.return_pe14(self.pe14.teardown())?;
    gpioe.return_pe15(self.pe15.teardown())?;

    Ok(())
  }
}

#[derive(PartialEq)]
enum WhichLed {
  Led3,
  Led4,
  Led5,
  Led6,
  Led7,
  Led8,
  Led9,
  Led10,
}
impl WhichLed {
  pub fn next(&mut self) -> WhichLed {
    match self {
      WhichLed::Led3 => WhichLed::Led5,
      WhichLed::Led4 => WhichLed::Led3,
      WhichLed::Led5 => WhichLed::Led7,
      WhichLed::Led6 => WhichLed::Led4,
      WhichLed::Led7 => WhichLed::Led9,
      WhichLed::Led8 => WhichLed::Led6,
      WhichLed::Led9 => WhichLed::Led10,
      WhichLed::Led10 => WhichLed::Led8,
    }
  }
}
