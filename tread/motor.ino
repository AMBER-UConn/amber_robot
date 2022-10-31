#include "motor.h"

void init_motor(const SMotor * motor) {
    pinMode(motor->pin_1, OUTPUT);
    pinMode(motor->pin_2, OUTPUT);
    pinMode(motor->en_pin, OUTPUT);

    set_direction(motor, FORWARD);
}

/// Set the direction of the motor
/// If In 1 is HIGH and In2 is LOW, it goes forwards
/// If In1 is LOW and IN2 is HIGH, it goes backwards
void set_direction(const SMotor * motor, MotorDir dir) {
    if (dir == FORWARD) {
        digitalWrite(motor->pin_1,  HIGH);
        digitalWrite(motor->pin_2,  LOW);
    } else {
        digitalWrite(motor->pin_1,  LOW);
        digitalWrite(motor->pin_2,  HIGH);
    }
}

/*
EN_A controls the speed, 0-255 (max velocity)
*/
void set_velocity(const SMotor * motor, unsigned int speed) {
    analogWrite(motor->en_pin, speed);
}