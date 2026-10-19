#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // Asignación de pines (puedes cambiar a1 por a2 si sigues tu esquema original)
    let mq5_pin = pins.a0.into_analog_input(&mut adc);
    let mq6_pin = pins.a1.into_analog_input(&mut adc);

    // --- Configuración inicial para el MQ-5 (Golpeado) ---
    let mut valor_filtrado_mq5: f32 = mq5_pin.analog_read(&mut adc) as f32;
    let factor_suavizado: f32 = 0.1; // 0.1 = Lento pero estable para absorber ruido térmico
    let offset_calibracion: i32 = 0; // Ajusta este valor para alinear la línea base

    loop {
        // 1. Lectura y procesamiento del MQ-5 (Metano/GLP)
        let lectura_bruta_mq5 = mq5_pin.analog_read(&mut adc) as f32;
        
        // Ecuación del filtro para absorber los cambios de temperatura
        valor_filtrado_mq5 = (factor_suavizado * lectura_bruta_mq5) + ((1.0 - factor_suavizado) * valor_filtrado_mq5);
        let lectura_final_mq5 = (valor_filtrado_mq5 as i32) + offset_calibracion;

        // 2. Lectura directa del MQ-6 (Propano)
        let lectura_mq6 = mq6_pin.analog_read(&mut adc);

        // 3. Impresión estructurada de ambos sensores para comparar
        ufmt::uwriteln!(
            &mut serial, 
            "MQ5 (Golpeado) -> Bruto: {} | Regulado: {}   ||   MQ6 (Intacto) -> ADC: {}", 
            lectura_bruta_mq5 as i32, 
            lectura_final_mq5,
            lectura_mq6
        ).unwrap();

        arduino_hal::delay_ms(500);
    }
}
