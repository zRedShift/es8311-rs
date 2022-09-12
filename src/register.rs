#[derive(Debug, Copy, Clone, strum::EnumIter)]
pub(crate) enum Register {
    ResetReg00 = 0x00, //reset digital,csm,clock manager etc.

    // Clock Scheme Register definition
    ClkManagerReg01 = 0x01, // select clk src for mclk, enable clock for codec
    ClkManagerReg02 = 0x02, // clk divider and clk multiplier
    ClkManagerReg03 = 0x03, // adc fsmode and osr
    ClkManagerReg04 = 0x04, // dac osr
    ClkManagerReg05 = 0x05, // clk divier for adc and dac
    ClkManagerReg06 = 0x06, // bclk inverter and divider
    ClkManagerReg07 = 0x07, // tri-state, lrck divider
    ClkManagerReg08 = 0x08, // lrck divider
    // SDP
    SdpInReg09 = 0x09,  // dac serial digital port
    SdpOutReg0A = 0x0A, // adc serial digital port
    // SYSTEM
    SystemReg0B = 0x0B, // system
    SystemReg0C = 0x0C, // system
    SystemReg0D = 0x0D, // system, power up/down
    SystemReg0E = 0x0E, // system, power up/down
    SystemReg0F = 0x0F, // system, low power
    SystemReg10 = 0x10, // system
    SystemReg11 = 0x11, // system
    SystemReg12 = 0x12, // system, Enable DAC
    SystemReg13 = 0x13, // system
    SystemReg14 = 0x14, // system, select DMIC, select analog pga gain
    // ADC
    AdcReg15 = 0x15, // ADC, adc ramp rate, dmic sense
    AdcReg16 = 0x16, // ADC
    AdcReg17 = 0x17, // ADC, volume
    AdcReg18 = 0x18, // ADC, alc enable and winsize
    AdcReg19 = 0x19, // ADC, alc maxlevel
    AdcReg1A = 0x1A, // ADC, alc automute
    AdcReg1B = 0x1B, // ADC, alc automute, adc hpf s1
    AdcReg1C = 0x1C, // ADC, equalizer, hpf s2
    // DAC
    DacReg31 = 0x31, // DAC, mute
    DacReg32 = 0x32, // DAC, volume
    DacReg33 = 0x33, // DAC, offset
    DacReg34 = 0x34, // DAC, drc enable, drc winsize
    DacReg35 = 0x35, // DAC, drc maxlevel, minilevel
    DacReg37 = 0x37, // DAC, ramprate
    // GPIO
    GpioReg44 = 0x44, // GPIO, dac2adc for test
    GpReg45 = 0x45,   // GP CONTROL
    // CHIP
    ChD1RegFD = 0xFD,  // CHIP ID1
    ChD2RegFE = 0xFE,  // CHIP ID2
    ChVerRegFF = 0xFF, // VERSION
}
