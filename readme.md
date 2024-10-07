# VREF Question

This repo creates an example to use VREF using embassy.

As of now, it doesn't appear to work.  The vref out pin remains low, and the ADC
reading gives 4095 (2^12-1).

To replicate, you can take a Nucleo-g0b1re and depopulate the 0 ohm resistor on
SB28.  This will disconnect VREF from VDD. and allow you to test the example.

This refence C code enables VREF correctly and results in a valid calibration:

```c
void
ADC_BSP_init(void)
{
    // Clear buffer used for DMA
    memset((void *)&ADC_BSP_samples_, 0xFF, sizeof(ADC_BSP_samples_));
    // Ref RM0444 15.3 ADC functional description
    // https://www.st.com/resource/en/reference_manual/rm0444-stm32g0x1-advanced-armbased-32bit-mcus-stmicroelectronics.pdf

    // Enable ADC clock tree
    STM_RCC->apbenr2.adcen = 1;

    ADC_BSP_enable_adc_voltage_regulator();
    ADC_BSP_calibrate();

    STM_ADC1->cfgr2.ckmode = ADC_ckmode_pclk_div2;
    MILLIS_delay_us(20);

    STM_ADC1->ccr.vbaten = 1;
    STM_ADC1->ccr.tsen   = 1;
    STM_ADC1->ccr.vrefen = 1;
    STM_ADC1->ccr.presc  = 1;
    ADC_BSP_start();
}

static void
ADC_BSP_enable_adc_voltage_regulator(void)
{
    // Ref RM0444 15.3.2 ADC voltage regulator (ADVREGEN)
    STM_ADC1->cr.advregen = 1;

    // Wait for ADC voltage to stabilize
    // Ref DS13560 Rev 5 STM32G0B1xB/xC/xE p. 101 table 62
    MILLIS_delay_us(20);
}

static void
ADC_BSP_calibrate(void)
{
    // Ref RM0444 15.3.3 Calibration (ADCAL) p.347
    assert(0 == STM_ADC1->cr.aden);
    assert(1 == STM_ADC1->cr.advregen);
    assert(0 == STM_ADC1->cfgr1.dmaen);

    STM_ADC1->cr.adcal = 1;
    while (0 == STM_ADC1->isr.eocal) {}
    STM_ADC1->isr.eocal = 1;

    // Calibration result is stored in `STM_ADC1->calfact`
}
```
