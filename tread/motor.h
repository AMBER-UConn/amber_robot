#ifndef MOTOR_H_   /* Include guard */
#define MOTOR_H_

typedef enum Direction {
    FORWARD = 0,
    BACKWARD = 1,
} MotorDir;

typedef struct Motor {
    unsigned int axis;
    unsigned int en_pin; // This ranges from 0-255 and controls the speed ot the motor
    bool pin_1; // 
    bool pin_2; //
} SMotor;

void init_motor(const SMotor * motor);
void set_direction(const SMotor * motor, MotorDir dir);
void set_velocity(const SMotor * motor, unsigned int speed);

#endif